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

Identifier: 0xC1

This one responds with "ff c1 da" when polled. This doesn't seem normal at all and when issuing bad commands while in escape mode, the response is 0xc1 no matter which controller I use. Also, the left and square buttons don't seem to work so it makes sense that it may be in an error state.

### DualShock (SCPH-1200) and DualShock 2 (SCPH-10010)

Lots of documentation on these guys (check the bibliography) and should be straightforward to support.

Identifier 0x41

Responds to polling with "ff 41 5a"

### DVD Remote Reciever (SCPH-10160)

This seems to need an initialization sequence as it won't report anything other than its usual device id (0x41) when first plugged in, no matter how much I mash the buttons. I'll have to hack my PS2 and sniff the bus for this one.

### Multitaps for PSX (SCPH-1070) and PS2 (SCPH-10090 and SCPH-70120)

I don't own either of these, so they're at the bottom of the list. After reading psxpblib, it looks like the multi-tap uses the first byte of the command message to address a particular port. Normally it's set to 1, so I assume I can reach ports A, B, C, and D with 1, 2, 3, and 4 respectively.
