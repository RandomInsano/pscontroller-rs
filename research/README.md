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

Fairly simple controller. While it doesn't have an escape mode it does respond to a fair number of commands more than the newer controllers. 

Responses to various commands through `scanner.rs` in the examples:

(Need to dump this)

### JogCon

Namco's force-feedback handheld controller

Identifier: 0xE

If you don't press any buttons on the controller for 60 seconds the wheel will disable itself until a button is pressed again. Ridge Racer games call this "Safety Mode". I haven't checked to see if there is a status bit set for this.

Responses to various commands through `scanner.rs` in the examples:

(Need to dump this)

### NeGcon

Namco's weird twisty controller

Identifier: 0x2

While testing the NeGcon protocol using the JogCon's secret emulation of it, there isn't a lot to go on. It seems to only answer polling requests and does not support the escape mode. The only results from scanning was this:

```text
Command 42: ff 23 5a ff ff 8a 00 00 00 ff
```

There are a large number of controllers that respond with an identifier of 0x2 and it makes automatic detection of this controller brittle versus say, the baton controller. How do we know which one has been plugged in for example? Neither have escape modes for constants, bot report the same number of bytes in their response. Mighty tricky I say!

#### Polling response data

The polling response is 3*16 bits long. The first 16 bits of the response are the usual buttons, followed by a signed 16bit number for the jog wheel's absolute position. The second last byte is the JogCon's current status while the last byte is always zero. The six different control values I saw while playing one lap in Ridge Racer V where:

```text
0x01 = Clockwise
0x02 = Counter Clockwise
0x04 = Max reached (either CW or CCW)
0xC0 = Unkonwn, occurs once
```

The command byte when the motor is enabled (same as dual shock) seems to be broken into two nybbles. The first is the mode, and the second is how hard to drive the motor (0 - 127).

```text
0x1 = Turn clockwise
0x2 = Turn counter clockwise
0x3 = Hold at position
0x8 = Unknown, is followed by 0xff five times, occured 27 times, drive value 0
0xB = Unknown, is followed by 0xff five times, occured once, drive value F
0xC = Unknown, is followed by 0xff five times, occured once, drive value 0
```

### GunCon (SCPH-0034)

Identifier 0x6

No responses for any commands other than polling. The output seems to only the usual 2 bytes for buttons, and there are two 16bit values for X and Y coordinates. X provides a range from around 93 to 455 and Y is a range between 25 to 230.

If the GunCon can't find the screen, it will send out X:10,Y:1 as the read coordinates. Also, it really does need the brightest picture possible to read well. Time Crisis sets the screen to white (in game) or yellow (calibration screen) while reading the GunCon's position to give it the best chance of picking it up.

Reading data up to 100KHz works just fine, but I'm not sure how often to poll is since my test setup was not ideal.

### DualShock (SCPH-1200)

Identifier 0x7

Responses to various commands through `scanner.rs` in the examples:

```text
(Cmd:40) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:41) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:42) ff 41 5a ff ff ff ff ff ff ff
(Cmd:43) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:44) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:45) ff f3 5a 01 02 00 02 01 00 ff
(Cmd:46) ff f3 5a 00 00 01 02 00 0a ff
(Cmd:47) ff f3 5a 00 00 02 00 01 00 ff
(Cmd:48) ff f3 5a 00 00 00 00 01 00 ff
(Cmd:49) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:4a) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:4c) ff f3 5a 00 00 00 04 00 00 ff
(Cmd:4d) ff f3 5a ff ff ff ff ff ff ff
(Cmd:4e) ff f3 5a 00 00 00 00 00 00 ff
```


### DualShock 2 (SCPH-10010)

Sony seems to have extended the command a little for this one as there is now the ability to specify the polling response (command 0x4f) and there is also some new stats or constant at command 0xa0.

Identifier 0x7

Responses to various commands through `scanner.rs` in the examples:

```text
(Cmd:40) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:41) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:42) ff f3 5a ff ff 89 85 79 8c ff
(Cmd:43) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:44) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:45) ff f3 5a 03 02 00 02 01 00 ff
(Cmd:46) ff f3 5a 00 00 01 02 00 0a ff
(Cmd:47) ff f3 5a 00 00 02 00 01 00 ff
(Cmd:48) ff f3 5a 00 00 00 00 01 00 ff
(Cmd:49) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:4a) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:4b) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:4c) ff f3 5a 00 00 00 04 00 00 ff
(Cmd:4d) ff f3 5a ff ff ff ff ff ff ff
(Cmd:4e) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:4f) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:a0) ff f3 5a 05 01 02 00 00 00 ff
```

### Guitar Hero Controller

This responds nearly identically to the DualShock 1, but supports the polling response customization command (0x4f). The buttons, strum, and star power switch all correspond to certain face buttons that I haven't mapped yet. The whammy bar corresponds to one of the analog axis of the DualShock 1.

The only way I can see to differentiate this controller from the PSX DualShock controller is to either use the unusal response to command 0x42 in escape mode, or the fact that it does respond to 0x4f. It will also take some restructuring of the library internally.

Responses to various commands through `scanner.rs` in the examples:

```text
(Cmd:40) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:41) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:42) ff f3 5a 7f ff 7f 7f 7f 7f ff
(Cmd:43) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:44) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:45) ff f3 5a 01 02 00 02 01 00 ff
(Cmd:46) ff f3 5a 00 00 01 02 00 0a ff
(Cmd:47) ff f3 5a 00 00 02 00 01 00 ff
(Cmd:48) ff f3 5a 00 00 00 00 01 00 ff
(Cmd:49) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:4a) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:4b) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:4c) ff f3 5a 00 00 00 04 00 00 ff
(Cmd:4d) ff f3 5a ff ff ff ff ff ff ff
(Cmd:4e) ff f3 5a 00 00 00 00 00 00 ff
(Cmd:4f) ff f3 5a 00 00 00 00 00 00 ff
```

### DVD Remote Reciever (SCPH-10160)

This device does not conform to the standard protocols that the other devices do. Near as I can tell, the additional 32 buttons on the device are encoded into the ACK responses. When performing a real-time dump of the SPI data, default buttons for the PlayStation (Circle, Square, etc) registered, but there was nothing from the DVD-specific buttons (Play, Paus, Stop). I did notice from logic analyser dump that there seemed to be exact timeing changes to the ACK line. The pull-down time was either 1us or 2us between bytes. I'll have to use an oscilliscope to trigger on the CS line to get any deeper on this one.

The remote dongle also will only answer to poll requests. It can't enter escape mode, and does not respond to any other commands. With the DVD software 2.10 installed on the PlayStation 2, the only command every sent or acknowledged is poll (0x42) which responds with 0x41

### Multitaps for PSX (SCPH-1070) and PS2 (SCPH-10090 and SCPH-70120)

The multi-tap uses the first byte of the message sent to the controller to address a particular port. Normally that bytes is set to 1. Ports A, B, C, and D correspond with 1, 2, 3, and 4 respectively.

At one point when testing, the multitap replied with `0xff, 0x80, 0x5a` during polling. I tried fuzzing by running a poll command against all addresses between zero and 255, but no luck there. I'd still like to do a read on a console while things are in use, but it looks like there really is nothing here other than a really fast SPI switcher.

### Mad Maestro Baton

Well, this one looks... Odd. It's a wand with a removable shaft. It only answers to polling requests and doens't support escape mode (checked via the `scanner` tool in this package). When it does respond it can only keep up at around 60Hz as any faster will return no bytes until data is ready again. It seems to be made by [G.A.E. Inc.](https://en.wikipedia.org/wiki/GAE_(company)) and contains a "GAE-1" custom chip and the internals are designed by Optec Co. Ltd.

Identifier: 0x2

The format looks like this:

```text
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

```text
Status:    01 02 01 01 01 00
Const 1.1: 00 04 03 01 1e
Const 1.2: 00 00 00 00 00
Const 2:   00 01 00 00 00
Const 3.1: 00 00 04 00 00
Const 3.2: 00 00 04 00 00
```

#### PlayStation DualShock 1:

```text
Status:    01 02 01 02 01 00
Const 1.1: 00 01 02 00 0a
Const 1.2: 00 01 01 01 14
Const 2:   00 02 00 01 00
Const 3.1: 00 00 04 00 00
Const 3.2: 00 00 04 00 00
```

#### BeamScope Dual Charger (Knockoff DualShock 1):

```text
Status:    01 02 00 02 01 00
Const 1.1: 00 01 02 00 0a
Const 1.2: 00 01 01 01 14
Const 2:   00 02 01 00 00
Const 3.1: 00 00 04 00 00
Const 3.2: 00 00 04 00 00
```

The fact that Const 2 here is a little off from the official controllers seems to show that it doesn't matter much.

#### PlayStation DualShock 2:

```text
Status:    03 02 01 02 01 00
Const 1.1: 00 01 02 00 0a
Const 1.2: 00 01 01 01 14
Const 2:   00 02 00 01 00
Const 3.1: 00 00 04 00 00
Const 3.2: 00 00 07 00 00
```

#### Guitar Hero Controller

```text
Status:    01 02 01 02 01 00
Const 1.1: 00 01 02 00 0a
Const 1.2: 00 01 01 01 14
Const 2:   00 02 00 01 00
Const 3.1: 00 00 04 00 00
Const 3.2: 00 00 07 00 00
```

#### Controller

```text
Commands failed
```

#### DVD Remote

```text
Commands failed
```

#### DDR Dance Mat

```text
Commands also failed. :'(
```

#### NeGcon (JogCon emulating the NeGcon)

```text
And more failing
```
