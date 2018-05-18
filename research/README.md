Hacking Notes
===============

Device types
-------------

Richard Davies who created the [PSX Peripheral Bus Library](http://www.debaser.force9.co.uk/psxcn/) over 20 years ago did a fantastic job catalogging controllers from the pre-PS2 era. I'm picking up this torch to fill in the rest, so feel free to reach out if you have some really weird controller device you want to lend me via the powers of UPS or FedEx.

| Device | JPN P/N    | EU/NA P/N  | Friendly Name    |
| ------ | ---------- | ---------- | -----------------|
| 1      | SCPH-1030  |            | Mouse            |
| 2      | SLPH-0001  | SLEH-0003  | Namco neGcon     |
|        | SLPH-0007  |            | Nasca Pachinco Handle (untested); Twist = Twist, TW = B |
|        | SLPH-0015  |            | Volume Controller (untested); Rotation = Twist, A = Start, B = A |
|        |            | SLEH-0005  | MadKatz Steering Wheel (twitchy) |
|        |            |            | Mad Maestro conductor baton |
| 3      |            |            | Konami Lightgun (untested) |
| 4      | SCPH-1010  | SCPH 1080  | Controller |
|        | SCPH-1110  |            | Analog Joystick - Digital Mode |
|        |            | SCPH-1180  | Analog Controller - Digital Mode |
|        | SCPH-1150  | SCPH-1200  | Dual Shock Analog Controller - Digital Mode |
|        |            | SLEH-0011  | Ascii Resident Evil Pad |
|        |            | SLEH-0004  | Namco Arcade Stick (untested) |
|        |            | SCPH-10160 | PlayStation 2 DVD Remote (default) |
| 5      | SCPH-1110  |            | Analog Joystick - Analog Mode (untested) |
|        |            | SCPH-1180  | Analog Controller - Analog Green Mode |
| 6      |            | SLEH-0007  | Namco G-con45 |
| 7      | SCPH-1150  | SCPH-1200  | Dual Shock Analog Controller - Analog Red Mode |
|        |            | SCPH-1180  | Analog Controller - Analog Red Mode |
|        |            | N/A        | RedOctane Guitar Hero Controller |
| 9      | SCPH-2000  | N/A        | [Keyboard / Mouse Adapter](http://www.psxdev.net/forum/viewtopic.php?f=54&t=140) |
| 14     |            | SLEH-0020  | Namco Jogcon |
| ??     |            | SCPH-10160 | PlayStation 2 DVD Remote |
| 12     | N/A        | N/A        | This may be a reserved error state |

Notes:
* Devices can have different operating modes and will report differently depending on the mode.
* The device ID is purposely not repeated for readability.
* Removed the "E" designation from the list official controllers. My guess is that different regions have different letters.

Deep dives into particular devices
-----------------------------------

### Controller (SCPH-1080)

Original controller without analog sticks.

Identifier: 0xC

This one responds with "ff c1 da" when polled. This doesn't seem normal at all and when issuing bad commands while in escape mode, the response is 0xc1 no matter which controller I use. Also, the left and square buttons don't seem to work so it makes sense that it may be in an error state.

### JogCon

Namco's force-feedback handheld controller

Identifier 0xE

This pre-dates the analog controllers from Sony, but answers to the same commands as the DualShocks. If you don't press any buttons on the controller for 60 seconds it will disable itself until a button is pressed again. Ridge Racer games call this "Safety Mode".

#### Polling response data

The polling response is 3*16 bits long. The first 16 bits of the response are the usual buttons, followed by a signed 16bit number for the jog wheel's absolute position. The second last byte is the JogCon's current status while the last byte is always zero. The six different control values I saw while playing one lap in Ridge Racer V where:

```
0x01 = Clockwise
0x02 = Counter Clockwise
0x04 = Max reached (either CW or CCW)
0xC0 = Unkonwn, occurs once
```

The command byte when the motor is enabled (same as dual shock) seems to be broken into two nybbles. The first is the mode, and the second is how hard to drive the motor (0 - 127).

```
0x1 = Turn clockwise
0x2 = Turn counter clockwise
0x3 = Hold at position
0x8 = Unknown, is followed by 0xff five times, occured 27 times, drive value 0
0xB = Unknown, is followed by 0xff five times, occured once, drive value F
0xC = Unknown, is followed by 0xff five times, occured once, drive value 0
```

### DualShock (SCPH-1200) and DualShock 2 (SCPH-10010)

Lots of documentation on these guys (check the bibliography) and should be straightforward to support.

Identifier 0x4

Responds to polling with "ff 41 5a"

Fuzzing Results: (DualShock 2)

```
Fuzz: 42 - ff 41 5a ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff 
Fuzz: c2 - ff 41 5a ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff 
```

### DVD Remote Reciever (SCPH-10160)

This device does not conform to the standard protocols that the other devices do. Near as I can tell, the additional 32 buttons on the device are encoded into the ACK responses. When performing a real-time dump of the SPI data, button presses registered, but I did notice the 1us pull-down time of the ACK line expand to 2us. I'll have to use an oscilliscope to trigger on the CS line to get any deeper on this one.

The remote dongle also will only answer to poll requests. It can't enter escape mode, and does not respond to any other commands. With the DVD software 2.10 installed on the PlayStation 2, the only command every sent or acknowledged is poll (0x42) which responds with 0x41

### Multitaps for PSX (SCPH-1070) and PS2 (SCPH-10090 and SCPH-70120)

The multi-tap uses the first byte of the command message to address a particular port. Normally it's set to 1, and ports A, B, C, and D with 1, 2, 3, and 4 respectively.

At one point when testing, the multitap replied with `0xff, 0x80, 0x5a` during polling. I'm not sure how to set it to that mode, but considering it works now I don't think I'll research it much further.

### Mad Maestro Baton

Well, this one looks... Odd. It's a wand with a removable shaft. It only answers to polling requests and doens't support escape mode (checked via the `scanner` tool in this package). When it does respond it can only keep up at around 60Hz as any faster will return no data at all. It seems to be made by [G.A.E. Inc.](https://en.wikipedia.org/wiki/GAE_(company)) and contains a "GAE-1" chip by Optec Co. Ltd.

Identifier: 0x2

The format looks like this:

```
ff 23 5a - ff ff 80 80 80 80
```

The first two response bytes are for buttons "A" (bit 4 of byte 1) and "B" (bit 2 of byte 2). The third byte seems to be the acceleration sensed, and the last three return nothing, but I'm expecting it to have been an accelerometer. I've opened it up and there are two metal shields. We'll see what's under those and see if I can fix/replace them.


Raw Data
-------------

### Extended status:

In:  01 45 00 | 5a 5a 5a 5a 5a 5a
Out: -- f3 5a | XX 02 YY 02 01 00 

YY = Analog mode

Guitar Hero:
    XX = 01, YY = 00
DualShock 1:
    XX = 01, YY = Analog Mode (0 = off, 1 = on)
DualShock 2:
    XX = 02, YY = Analog Mode (0 = off, 1 = on)
Controller:
    Didn't respond to request
DVD Remote:
    Didn't respond either



### Data Dumps:

#### JogCon

```
Status:    01 02 01 01 01 00 
Const 1.1: 00 04 03 01 1e 
Const 1.2: 00 00 00 00 00 
Const 2:   00 01 00 00 00 
Const 3.1: 00 00 04 00 00 
Const 3.2: 00 00 04 00 00 
```

#### PlayStation DualShock 1:

```
Status:    01 02 01 02 01 00
Const 1.1: 00 01 02 00 0a
Const 1.2: 00 01 01 01 14
Const 2:   00 02 00 01 00
Const 3.1: 00 00 04 00 00
Const 3.2: 00 00 04 00 00
```

#### BeamScope Dual Charger (Knockoff DualShock 1):

```
Status:    01 02 00 02 01 00
Const 1.1: 00 01 02 00 0a
Const 1.2: 00 01 01 01 14
Const 2:   00 02 01 00 00
Const 3.1: 00 00 04 00 00
Const 3.2: 00 00 04 00 00
```

#### PlayStation DualShock 2:

```
Status:    03 02 01 02 01 00
Const 1.1: 00 01 02 00 0a
Const 1.2: 00 01 01 01 14
Const 2:   00 02 00 01 00
Const 3.1: 00 00 04 00 00
Const 3.2: 00 00 07 00 00
```

#### Guitar Hero Controller

```
Status:    01 02 01 02 01 00
Const 1.1: 00 01 02 00 0a
Const 1.2: 00 01 01 01 14
Const 2:   00 02 00 01 00
Const 3.1: 00 00 04 00 00
Const 3.2: 00 00 07 00 00
```

#### Controller

```
Commands failed
```

#### DVD Remote

```
Commands failed
```

#### DDR Dance Mat

```
Commands also failed. :'(
```
