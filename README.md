PlayStation Controller (PAD) Driver for Rust Embedded
========================================================

This project is an attempt to support any PlayStation controller device for the PlayStation 1 or 2 consoles via the SPI port on any platform which support's Rust's [embedded-hal](https://github.com/japaric/embedded-hal) abstraction layer. The main system this library is tested on is a Raspberry Pi 3 via the SPI bus on the board.

So far quite a few devices are supported, and random notes are kept in the `research` folder in this project. Here's the current list:

* Original PlayStation digital mode
* DualShock 1/2 analog sticks
* DualShock 2 pressure sensitive buttons
* DualShock rumble
* Guitar Hero controller
* JogCon
* JogCon force feedback
* NegCon
* PSX Multitap (SCPH-1070)

Most PlayStation games assume the controller which is plugged in, so it's difficult to automatically detect which controller is being used automatically. Currently, there is a best-effort based on a 16-value identifier, but there are heavy overlaps. For example, there is no conventient way to differentiate a Guitar Hero controller from an original Dual Shock controller. Because of this and the design of the library, there are a number of controllers that this library cannot support until a refactoring is done.

This project is immediately usable today, but work needs to be done to increase error detection and allow the ergonimc use of un-detectable controllers. There are examples to go on to learn how to use this library and discover some secrets of the PlayStation controllers you might own.

If you want to contribute new controller info or are having trouble wiring up your devices, open an issue and we can help you out. The bibliography below should also be a good jumping-off point as many of those authors are better writers. 😃

Bibliography
---------------

* [psxpad.html](http://domisan.sakura.ne.jp/article/psxpad/psxpad.html) - Wiring, testing and bootstrapping game pad on Linux with SPI
* [ps_eng.txt](http://kaele.com/~kashima/games/ps_eng.txt) - Controller / Memory Card Protocols (pre-DualShock)
* [Playstation 2 (Dual Shock) controller protocol notes](https://gist.github.com/scanlime/5042071) - Command protocols
* [psxpblib](http://www.debaser.force9.co.uk/psxpblib/) - Interfacing PlayStation controllers via the parallel port
* [Simulated PS2 Controller for Autonomously playing Guitar Hero](http://procrastineering.blogspot.ca/2010/12/simulated-ps2-controller-for.html) - SPI protocol captures
