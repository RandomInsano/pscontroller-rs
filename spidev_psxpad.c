/*
 * PSX(Play Station 1/2) pad testing utility (using spidev driver)
 *
 * Copyright (c) 2017 AZO
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License.
 */

#include <stdint.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>
#include <getopt.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <linux/types.h>
#include <linux/spi/spidev.h>

#define PSXPAD_MAXPADNUM 2
#if PSXPAD_MAXPADNUM == 0 || PSXPAD_MAXPADNUM > 2
#error PSXPAD_MAXPADNUM must be 1-2
#endif

typedef enum {
	PSXPAD_KEYSTATE_DIGITAL = 0,
	PSXPAD_KEYSTATE_ANALOG1,
	PSXPAD_KEYSTATE_ANALOG2,
	PSXPAD_KEYSTATE_UNKNOWN
} PSXPad_KeyStateType_t;

typedef struct PSXPad_KeyState {
	PSXPad_KeyStateType_t tType;
	/* PSXPAD_KEYSTATE_DIGITAL */
	uint8_t bSel;
	uint8_t bStt;
	uint8_t bU;
	uint8_t bR;
	uint8_t bD;
	uint8_t bL;
	uint8_t bL2;
	uint8_t bR2;
	uint8_t bL1;
	uint8_t bR1;
	uint8_t bTri;
	uint8_t bCir;
	uint8_t bCrs;
	uint8_t bSqr;
	/* PSXPAD_KEYSTATE_ANALOG1 */
	uint8_t bL3;
	uint8_t bR3;
	uint8_t u8RX;
	uint8_t u8RY;
	uint8_t u8LX;
	uint8_t u8LY;
	/* PSXPAD_KEYSTATE_ANALOG2 */
	uint8_t u8AR;
	uint8_t u8AL;
	uint8_t u8AU;
	uint8_t u8AD;
	uint8_t u8ATri;
	uint8_t u8ACir;
	uint8_t u8ACrs;
	uint8_t u8ASqr;
	uint8_t u8AL1;
	uint8_t u8AR1;
	uint8_t u8AL2;
	uint8_t u8AR2;
} PSXPad_KeyState_t;

static const char device[] = "/dev/spidev0.0";

const uint8_t PSX_CMD_INIT_PRESSURE[]	= {0x01, 0x40, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00};
const uint8_t PSX_CMD_POLL[]		= {0x01, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00};
const uint8_t PSX_CMD_ENTER_CFG[]	= {0x01, 0x43, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00};
const uint8_t PSX_CMD_EXIT_CFG[]	= {0x01, 0x43, 0x00, 0x00, 0x5A, 0x5A, 0x5A, 0x5A, 0x5A};
const uint8_t PSX_CMD_ENABLE_MOTOR[]	= {0x01, 0x4D, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF};
const uint8_t PSX_CMD_ALL_PRESSURE[]	= {0x01, 0x4F, 0x00, 0xFF, 0xFF, 0x03, 0x00, 0x00, 0x00};
const uint8_t PSX_CMD_AD_MODE[]		= {0x01, 0x44, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00};

typedef struct PSXPad {
	uint8_t lu8PoolCmd[sizeof(PSX_CMD_POLL)];
	uint8_t lu8Response[sizeof(PSX_CMD_POLL)];
	uint8_t u8AttPinNo;
	uint8_t bAnalog;
	uint8_t bLock;
	uint8_t bMotor1Enable;
	uint8_t bMotor2Enable;
	uint8_t u8Motor1Level;
	uint8_t u8Motor2Level;
	uint8_t lu8EnableMotor[sizeof(PSX_CMD_ENABLE_MOTOR)];
	uint8_t lu8ADMode[sizeof(PSX_CMD_AD_MODE)];
} PSXPad_t;

typedef struct PSXPads {
	int iFD;
	struct spi_ioc_transfer tTransfer;
	uint8_t u8PadsNum;
	PSXPad_t ltPad[PSXPAD_MAXPADNUM];
} PSXPads_t;

static PSXPads_t tPSXPads;
static PSXPad_KeyState_t tPSXKeyState;

#define REVERSE_BIT(x) ((((x) & 0x80) >> 7) | (((x) & 0x40) >> 5) | (((x) & 0x20) >> 3) | (((x) & 0x10) >> 1) | (((x) & 0x08) << 1) | (((x) & 0x04) << 3) | (((x) & 0x02) << 5) | (((x) & 0x01) << 7))

static void pabort(const char s[])
{
	perror(s);
	abort();
}

int spi0_init(int i_iFD, struct spi_ioc_transfer* o_ptTransfer, const uint8_t i_u8Mode, const uint8_t i_u8Bits, const uint32_t i_u32Speed, const uint16_t i_u16Delay) {
	int ret = 0;

	if(!o_ptTransfer)
		return -1;

	/*
	 * spi mode
	 */
	ret = ioctl(i_iFD, SPI_IOC_WR_MODE, &i_u8Mode);
	if (ret == -1)
		pabort("can't set write spi mode");
	ret = ioctl(i_iFD, SPI_IOC_RD_MODE, &i_u8Mode);
	if (ret == -1)
		pabort("can't get read spi mode");

	/*
	 * bits per word
	 */
	ret = ioctl(i_iFD, SPI_IOC_WR_BITS_PER_WORD, &i_u8Bits);
	if (ret == -1)
		pabort("can't set write bits per word");
	ret = ioctl(i_iFD, SPI_IOC_RD_BITS_PER_WORD, &i_u8Bits);
	if (ret == -1)
		pabort("can't get read bits per word");

	/*
	 * max speed hz
	 */
	ret = ioctl(i_iFD, SPI_IOC_WR_MAX_SPEED_HZ, &i_u32Speed);
	if (ret == -1)
		pabort("can't set write max speed hz");
	ret = ioctl(i_iFD, SPI_IOC_RD_MAX_SPEED_HZ, &i_u32Speed);
	if (ret == -1)
		pabort("can't get read max speed hz");

	printf("spi mode: %d\n", i_u8Mode);
	printf("bits per word: %d\n", i_u8Bits);
	printf("max speed: %d Hz (%d KHz)\n", i_u32Speed, i_u32Speed/1000);

	/* set transfer settings */
	o_ptTransfer->delay_usecs = i_u16Delay;
	o_ptTransfer->speed_hz = i_u32Speed;
	o_ptTransfer->bits_per_word = i_u8Bits;

	return ret;
}

void PSXPads_Init(PSXPads_t* ptPSXPads, const char i_strDevice[], const uint8_t i_u8PadNum) {
	uint8_t u8PadNo, u8Loc;

	if(!ptPSXPads)
		return;
	if(!i_strDevice)
		return;
	if(i_u8PadNum == 0 || i_u8PadNum >= PSXPAD_MAXPADNUM)
		return;

	ptPSXPads->iFD = open(i_strDevice, O_RDWR);
	if (ptPSXPads->iFD < 0)
		pabort("can't open device");

	/* mode 3, 125kbps */
	if(spi0_init(ptPSXPads->iFD, &(ptPSXPads->tTransfer), SPI_MODE_3, 8, 125000, 100) < 0)
		pabort("can't init SPI");

	ptPSXPads->u8PadsNum = i_u8PadNum;

	for(u8PadNo = 0; u8PadNo < ptPSXPads->u8PadsNum; u8PadNo++) {
		for(u8Loc = 0; u8Loc < sizeof(PSX_CMD_POLL); u8Loc++)
			ptPSXPads->ltPad[u8PadNo].lu8PoolCmd[u8Loc] = PSX_CMD_POLL[u8Loc];
		for(u8Loc = 0; u8Loc < sizeof(PSX_CMD_ENABLE_MOTOR); u8Loc++)
			ptPSXPads->ltPad[u8PadNo].lu8EnableMotor[u8Loc] = PSX_CMD_ENABLE_MOTOR[u8Loc];
		for(u8Loc = 0; u8Loc < sizeof(PSX_CMD_AD_MODE); u8Loc++)
			ptPSXPads->ltPad[u8PadNo].lu8ADMode[u8Loc] = PSX_CMD_AD_MODE[u8Loc];
	}
}

void PSXPads_Uninit(PSXPads_t* ptPSXPads) {
	if(!ptPSXPads)
		return;

	close(ptPSXPads->iFD);
}

void PSXPads_Command(PSXPads_t* ptPSXPads, const uint8_t u8PadNo, const uint8_t i_lu8SendCmd[], uint8_t o_lu8Response[], const uint8_t i_u8SendCmdLen) {
	int ret;
	uint8_t u8Loc;
	uint8_t u8SendBuf[0x100];

	if(!ptPSXPads)
		return;
	if(u8PadNo >= ptPSXPads->u8PadsNum)
		return;
	if(!i_lu8SendCmd)
		return;
	if(!o_lu8Response)
		return;
	if(i_u8SendCmdLen == 0)
		return;

	for(u8Loc = 0; u8Loc < i_u8SendCmdLen; u8Loc++)
		u8SendBuf[u8Loc] = REVERSE_BIT(i_lu8SendCmd[u8Loc]);

	/* set transfer settings */
	ptPSXPads->tTransfer.tx_buf	= (unsigned long)u8SendBuf;
	ptPSXPads->tTransfer.rx_buf	= (unsigned long)o_lu8Response;
	ptPSXPads->tTransfer.len	= i_u8SendCmdLen;
	ptPSXPads->tTransfer.cs_change	= u8PadNo;

	ret = ioctl(ptPSXPads->iFD, SPI_IOC_MESSAGE(1), &(ptPSXPads->tTransfer));
	if (ret < 1)
		pabort("can't send spi message");

	printf("\nData: ");
	for(u8Loc = 0; u8Loc < i_u8SendCmdLen; u8Loc++) {
		o_lu8Response[u8Loc] = REVERSE_BIT(o_lu8Response[u8Loc]);
		printf("%0x ", o_lu8Response[u8Loc]);
	}

	//sleep(1);
}

void PSXPads_Pool(PSXPads_t* ptPSXPads) {
	uint8_t u8PadNo;

	if(!ptPSXPads)
		return;

	for(u8PadNo = 0; u8PadNo < ptPSXPads->u8PadsNum; u8PadNo++)
		PSXPads_Command(ptPSXPads, u8PadNo, ptPSXPads->ltPad[u8PadNo].lu8PoolCmd, ptPSXPads->ltPad[u8PadNo].lu8Response, sizeof(PSX_CMD_POLL));
}

void PSXPads_SetADMode(PSXPads_t* ptPSXPads, const uint8_t u8PadNo, const uint8_t i_bAnalog, const uint8_t i_bLock) {
	if(!ptPSXPads)
		return;
	if(u8PadNo >= ptPSXPads->u8PadsNum)
		return;

	ptPSXPads->ltPad[u8PadNo].bAnalog = i_bAnalog ? 1 : 0;
	ptPSXPads->ltPad[u8PadNo].bLock   = i_bLock   ? 1 : 0;

	ptPSXPads->ltPad[u8PadNo].lu8ADMode[3] = ptPSXPads->ltPad[u8PadNo].bAnalog ? 0x01 : 0x00;
	ptPSXPads->ltPad[u8PadNo].lu8ADMode[4] = ptPSXPads->ltPad[u8PadNo].bLock   ? 0x03 : 0x00;

	PSXPads_Command(ptPSXPads, u8PadNo, PSX_CMD_ENTER_CFG, ptPSXPads->ltPad[u8PadNo].lu8Response, sizeof(PSX_CMD_ENTER_CFG));
	PSXPads_Command(ptPSXPads, u8PadNo, ptPSXPads->ltPad[u8PadNo].lu8ADMode, ptPSXPads->ltPad[u8PadNo].lu8Response, sizeof(PSX_CMD_AD_MODE));
	PSXPads_Command(ptPSXPads, u8PadNo, PSX_CMD_INIT_PRESSURE, ptPSXPads->ltPad[u8PadNo].lu8Response, sizeof(PSX_CMD_INIT_PRESSURE));
	PSXPads_Command(ptPSXPads, u8PadNo, PSX_CMD_ALL_PRESSURE, ptPSXPads->ltPad[u8PadNo].lu8Response, sizeof(PSX_CMD_ALL_PRESSURE));
	PSXPads_Command(ptPSXPads, u8PadNo, PSX_CMD_EXIT_CFG, ptPSXPads->ltPad[u8PadNo].lu8Response, sizeof(PSX_CMD_EXIT_CFG));
}

void PSXPads_SetEnableMotor(PSXPads_t* ptPSXPads, const uint8_t u8PadNo, const uint8_t i_bMotor1Enable, const uint8_t i_bMotor2Enable) {
	if(!ptPSXPads)
		return;
	if(u8PadNo >= ptPSXPads->u8PadsNum)
		return;

	ptPSXPads->ltPad[u8PadNo].bMotor1Enable = i_bMotor1Enable ? 1 : 0;
	ptPSXPads->ltPad[u8PadNo].bMotor2Enable = i_bMotor2Enable ? 1 : 0;

	ptPSXPads->ltPad[u8PadNo].lu8EnableMotor[3] = ptPSXPads->ltPad[u8PadNo].bMotor1Enable ? 0x00 : 0xFF;
	ptPSXPads->ltPad[u8PadNo].lu8EnableMotor[4] = ptPSXPads->ltPad[u8PadNo].bMotor2Enable ? 0x01 : 0xFF;

	PSXPads_Command(ptPSXPads, u8PadNo, PSX_CMD_ENTER_CFG, ptPSXPads->ltPad[u8PadNo].lu8Response, sizeof(PSX_CMD_ENTER_CFG));
	PSXPads_Command(ptPSXPads, u8PadNo, ptPSXPads->ltPad[u8PadNo].lu8EnableMotor, ptPSXPads->ltPad[u8PadNo].lu8Response, sizeof(PSX_CMD_ENABLE_MOTOR));
	PSXPads_Command(ptPSXPads, u8PadNo, PSX_CMD_EXIT_CFG, ptPSXPads->ltPad[u8PadNo].lu8Response, sizeof(PSX_CMD_EXIT_CFG));
}

void PSXPads_SetMotorLevel(PSXPads_t* ptPSXPads, const uint8_t u8PadNo, const uint8_t i_u8Motor1Level, const uint8_t i_u8Motor2Level) {
	if(!ptPSXPads)
		return;
	if(u8PadNo >= ptPSXPads->u8PadsNum)
		return;

 	ptPSXPads->ltPad[u8PadNo].u8Motor1Level = i_u8Motor1Level ? 0xFF : 0x00;
	ptPSXPads->ltPad[u8PadNo].u8Motor2Level = i_u8Motor2Level;

	ptPSXPads->ltPad[u8PadNo].lu8PoolCmd[3] = ptPSXPads->ltPad[u8PadNo].u8Motor1Level;
	ptPSXPads->ltPad[u8PadNo].lu8PoolCmd[4] = ptPSXPads->ltPad[u8PadNo].u8Motor2Level;
}

void PSXPads_GetKeyState(PSXPads_t* ptPSXPads, const uint8_t u8PadNo, PSXPad_KeyState_t* o_ptKeyState) {
	if(!ptPSXPads)
		return;
	if(u8PadNo >= ptPSXPads->u8PadsNum)
		return;
	if(!o_ptKeyState)
		return;

	o_ptKeyState->tType = PSXPAD_KEYSTATE_UNKNOWN;

	switch(ptPSXPads->ltPad[u8PadNo].lu8Response[1]) {
	case 0x79:
		o_ptKeyState->tType = PSXPAD_KEYSTATE_ANALOG2;
		o_ptKeyState->u8AR   = ptPSXPads->ltPad[u8PadNo].lu8Response[ 9];
		o_ptKeyState->u8AL   = ptPSXPads->ltPad[u8PadNo].lu8Response[10];
		o_ptKeyState->u8AU   = ptPSXPads->ltPad[u8PadNo].lu8Response[11];
		o_ptKeyState->u8AD   = ptPSXPads->ltPad[u8PadNo].lu8Response[12];
		o_ptKeyState->u8ATri = ptPSXPads->ltPad[u8PadNo].lu8Response[13];
		o_ptKeyState->u8ACir = ptPSXPads->ltPad[u8PadNo].lu8Response[14];
		o_ptKeyState->u8ACrs = ptPSXPads->ltPad[u8PadNo].lu8Response[15];
		o_ptKeyState->u8ASqr = ptPSXPads->ltPad[u8PadNo].lu8Response[16];
		o_ptKeyState->u8AL1  = ptPSXPads->ltPad[u8PadNo].lu8Response[17];
		o_ptKeyState->u8AR1  = ptPSXPads->ltPad[u8PadNo].lu8Response[18];
		o_ptKeyState->u8AL2  = ptPSXPads->ltPad[u8PadNo].lu8Response[19];
		o_ptKeyState->u8AR2  = ptPSXPads->ltPad[u8PadNo].lu8Response[20];
	case 0x73:
		if(o_ptKeyState->tType == PSXPAD_KEYSTATE_UNKNOWN)
			o_ptKeyState->tType = PSXPAD_KEYSTATE_ANALOG1;
		o_ptKeyState->u8RX = ptPSXPads->ltPad[u8PadNo].lu8Response[5];
		o_ptKeyState->u8RY = ptPSXPads->ltPad[u8PadNo].lu8Response[6];
		o_ptKeyState->u8LX = ptPSXPads->ltPad[u8PadNo].lu8Response[7];
		o_ptKeyState->u8LY = ptPSXPads->ltPad[u8PadNo].lu8Response[8];
		o_ptKeyState->bL3  = (ptPSXPads->ltPad[u8PadNo].lu8Response[3] & 0x02U) ? 0 : 1;
		o_ptKeyState->bR3  = (ptPSXPads->ltPad[u8PadNo].lu8Response[3] & 0x04U) ? 0 : 1;
	case 0x41:
		if(o_ptKeyState->tType == PSXPAD_KEYSTATE_UNKNOWN)
			o_ptKeyState->tType = PSXPAD_KEYSTATE_DIGITAL;
		o_ptKeyState->bSel = (ptPSXPads->ltPad[u8PadNo].lu8Response[3] & 0x01U) ? 0 : 1;
		o_ptKeyState->bStt = (ptPSXPads->ltPad[u8PadNo].lu8Response[3] & 0x08U) ? 0 : 1;
		o_ptKeyState->bU   = (ptPSXPads->ltPad[u8PadNo].lu8Response[3] & 0x10U) ? 0 : 1;
		o_ptKeyState->bR   = (ptPSXPads->ltPad[u8PadNo].lu8Response[3] & 0x20U) ? 0 : 1;
		o_ptKeyState->bD   = (ptPSXPads->ltPad[u8PadNo].lu8Response[3] & 0x40U) ? 0 : 1;
		o_ptKeyState->bL   = (ptPSXPads->ltPad[u8PadNo].lu8Response[3] & 0x80U) ? 0 : 1;
		o_ptKeyState->bL2  = (ptPSXPads->ltPad[u8PadNo].lu8Response[4] & 0x01U) ? 0 : 1;
		o_ptKeyState->bR2  = (ptPSXPads->ltPad[u8PadNo].lu8Response[4] & 0x02U) ? 0 : 1;
		o_ptKeyState->bL1  = (ptPSXPads->ltPad[u8PadNo].lu8Response[4] & 0x04U) ? 0 : 1;
		o_ptKeyState->bR1  = (ptPSXPads->ltPad[u8PadNo].lu8Response[4] & 0x08U) ? 0 : 1;
		o_ptKeyState->bTri = (ptPSXPads->ltPad[u8PadNo].lu8Response[4] & 0x10U) ? 0 : 1;
		o_ptKeyState->bCir = (ptPSXPads->ltPad[u8PadNo].lu8Response[4] & 0x20U) ? 0 : 1;
		o_ptKeyState->bCrs = (ptPSXPads->ltPad[u8PadNo].lu8Response[4] & 0x40U) ? 0 : 1;
		o_ptKeyState->bSqr = (ptPSXPads->ltPad[u8PadNo].lu8Response[4] & 0x80U) ? 0 : 1;
	}
}

int main(void) {
	int ret = 0;

	printf("Init\n");
	PSXPads_Init(&tPSXPads, device, 1);
	printf("Set mode\n");
	PSXPads_SetADMode(&tPSXPads, 0, 1, 1);

	while(1) {
		int i;
		PSXPads_Pool(&tPSXPads);
		PSXPads_GetKeyState(&tPSXPads, 0, &tPSXKeyState);

		//for(i=0;i<21;i++)
		//	printf("%02X ", tPSXPads.ltPad[0].lu8Response[i]);
		//printf("\n");

		usleep(16666);
		//sleep(1);
	}

	PSXPads_Uninit(&tPSXPads);

	return ret;
}

