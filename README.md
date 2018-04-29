PlayStation Controller (PAD) Driver for Rust Embedded
========================================================

The primary goal here is to support the DualShock 1/2 controllers as they're the most prevelent, but there are other fun control devices to try. Currently supported:

* [x] Original PlayStation digital mode
* [x] DualShock 1/2 analog sticks
* [x] DualShock 2 pressure sensitive buttons
* [x] JogCon
* [x] JogCon force feedback
* [x] NegCon
* [ ] DualShock rumble
* [ ] Guitar Hero guitar
* [ ] Dual Analog
* [ ] Multitaps
* [x] ~~DVD Remote (doesn't follow spec)~~

Notes for hacking status of these can be found in the 'research' folder.


Bibliography
---------------
* [psxpad.html](http://domisan.sakura.ne.jp/article/psxpad/psxpad.html) - Wiring, testing and bootstrapping game pad on Linux with SPI
* [ps_eng.txt](http://kaele.com/~kashima/games/ps_eng.txt) - Controller / Memory Card Protocols (pre-DualShock)
* [Playstation 2 (Dual Shock) controller protocol notes](https://gist.github.com/scanlime/5042071) - Command protocols
* [psxpblib](http://www.debaser.force9.co.uk/psxpblib/) - Interfacing PlayStation controllers via the parallel port
* [Simulated PS2 Controller for Autonomously playing Guitar Hero](http://procrastineering.blogspot.ca/2010/12/simulated-ps2-controller-for.html) - SPI protocol captures
