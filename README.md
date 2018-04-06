PlayStation Controller (PAD) Driver for Rust Embedded
========================================================

The primary goal here is to support the DualShock 1/2 controllers as they're the most prevelent, but there are other fun control devices to try. Currently supported:

* [ ] Original PlayStation digital mode
* [ ] DualShock 1/2 analog sticks
* [ ] DualShock pressure sensitive buttons
* [ ] Dual Analog
* [ ] DVD Remote
* [ ] Multitaps

Notes for hacking status of of these below.


### DualShock (SCPH-1200) and DualShock 2 (SCPH-10010)

Lots of documentation on these guys (check the bibliography) and should be straightforward to support.

### DVD Remote Reciever (SCPH-10160)

This seems to need an initialization sequence as it won't report anything other than its usual device id (0x41) when first plugged in. I'll have to hack my PS2 and sniff the bus for this one.

### Multitaps for PSX (SCPH-1070) and PS2 (SCPH-10090 and SCPH-70120)

I don't own either of these, so they're at the bottom of the list. I'm assuming it's just a special multiplexer, but I'll have to find one to fiture it out.

Bibliography
---------------
* [psxpad.html](http://domisan.sakura.ne.jp/article/psxpad/psxpad.html) - Wiring, testing and bootstrapping game pad on Linux with SPI
* [ps_eng.txt](http://kaele.com/~kashima/games/ps_eng.txt) - Controller / Memory Card Protocols (pre-DualShock)
* [Playstation 2 (Dual Shock) controller protocol notes](https://gist.github.com/scanlime/5042071) - Command protocols

