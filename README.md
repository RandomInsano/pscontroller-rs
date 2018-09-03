# PlayStation Controller (PAD) Driver for Rust Embedded

This project aims to support all controller devices for the PlayStation 1 or 2
consoles via an SPI port on any platform which support's Rust's
[embedded-hal](https://github.com/japaric/embedded-hal) abstraction layer. The
main system this library is tested on is a Raspberry Pi 3.

So far quite a few devices are supported! Here's the current list:

* Original PlayStation digital mode
* DualShock 1/2 analog sticks
* DualShock 2 pressure sensitive buttons
* DualShock rumble
* Guitar Hero controller *
* Mad Maestro baton *
* JogCon
* JogCon force feedback
* GunCon
* NegCon
* PSX Multitap (SCPH-1070)

*=Most PlayStation games can assume the type of controller which is plugged in,
but there are only 16 possible identifiers that can be used. In pracice there is
overlap and old devices do not contain advanced polling respones so the devices
marked above need to be manually requested.

This project is immediately usable today, but work needs to be done to increase
error detection and allow better of un-detectable controllers. There are
examples to go on to learn how to use this library and you can use the
`research` folder's documents to discover some secrets of the PlayStation
controllers you might own, though the bibliography below is also a good
jumping-off point.

If you want to contribute new controller info or are having trouble wiring up
your devices, open an issue and we can help you out.

## Bibliography

* [psxpad.html](http://domisan.sakura.ne.jp/article/psxpad/psxpad.html) - Wiring, testing and bootstrapping game pad on Linux with SPI
* [ps_eng.txt](http://kaele.com/~kashima/games/ps_eng.txt) - Controller / Memory Card Protocols (pre-DualShock)
* [Playstation 2 (Dual Shock) controller protocol notes](https://gist.github.com/scanlime/5042071) - Command protocols
* [psxpblib](http://www.debaser.force9.co.uk/psxpblib/) - Interfacing PlayStation controllers via the parallel port
* [Simulated PS2 Controller for Autonomously playing Guitar Hero](http://procrastineering.blogspot.ca/2010/12/simulated-ps2-controller-for.html) - SPI protocol captures
