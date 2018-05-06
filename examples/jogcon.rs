//! Control the JogCon
//! ======================
//! This example sends a control command to the JogCon, but at the moment
//! the API is fairly useless. Because the polling command is the one that
//! sets the JogCon's wheel mode, any time you read the buttons you will
//! disable the current control. That means this library needs to change
//! to accomodate that, but for now you can wait a few seconds when mashing
//! some buttons at least. :)

#![feature(duration_from_micros)]

extern crate linux_embedded_hal as linux_hal;
extern crate embedded_hal;
extern crate pscontroller_rs;

use std::io;
use linux_hal::Spidev;
use linux_hal::spidev::{SpidevOptions, SPI_MODE_3};
use linux_hal::Pin;

use pscontroller_rs::{
	PlayStationPort,
	Device,
	jogcon::{
		JogControl,
		ControlJC
	}
};

// Specific to the host device used on Linux, you'll have to change the following
// parameters depending on your board and also export and allow writing to the GPIO
const SPI_DEVICE: &str = "/dev/spidev0.0";
const SPI_SPEED: u32 = 10_000;

fn build_spi() -> io::Result<Spidev> {
	let mut spi = Spidev::open(SPI_DEVICE)?;
	let opts = SpidevOptions::new()
		.bits_per_word(8)
		.max_speed_hz(SPI_SPEED)
		.mode(SPI_MODE_3)
		.build();
	spi.configure(&opts)?;

	Ok(spi)
}

fn main() {
    let spi = build_spi().unwrap();
    let mut psp = PlayStationPort::new(spi, None::<Pin>);
	let mut control_jc = ControlJC::new(JogControl::Stop, 15);

	psp.enable_jogcon()
		.expect("Had trouble initializing the JogCon. Check /dev/spi* permissions.");

	println!("Use square, triangle, circle, left, right and up to control the JogCon");

	loop {
		let controller = match psp.read_input(Some(&control_jc)) {
			Ok(x) => x,
			Err(_) => continue
		};

		// We only care about the JogCon here so skip everything else
		let jogcon = match controller {
			Device::JogCon(x) => (x),
			_ => continue
		};

		// Control the jog wheel with the face buttons.
		if jogcon.buttons.square() {
			println!("    Left...  ");
			control_jc.mode = JogControl::Left;
		} else if jogcon.buttons.triangle() {
			println!("    Hold...  ");
			control_jc.mode = JogControl::Hold;
		} else if jogcon.buttons.circle() {
			println!("    Right... ");
			control_jc.mode = JogControl::Right;
		} else if jogcon.buttons.left() {
			println!("    Dropped revolution count ");
			control_jc.mode = JogControl::DropRevolutions;
		} else if jogcon.buttons.up() {
			println!("    Dropped revolution count and returning... ");
			control_jc.mode = JogControl::DropAndHold;
		} else if jogcon.buttons.right() {
			println!("    Set new hold position ");
			control_jc.mode = JogControl::NewHold;
		} else {
			// Skip the pause that's coming up
			control_jc.mode = JogControl::Stop;
		}
	}
}
