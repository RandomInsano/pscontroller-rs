extern crate linux_embedded_hal as linux_hal;
extern crate bit_reverse;
extern crate pscontroller_rs;

use std::io;
use linux_hal::Spidev;
use linux_hal::spidev::{SpidevOptions, SPI_MODE_3};

use pscontroller_rs::PlayStationPort;

fn build_spi() -> io::Result<Spidev> {
	let mut spi = Spidev::open("/dev/spidev32766.0")?;
	let opts = SpidevOptions::new()
		.bits_per_word(8)
		.max_speed_hz(50_000)
		.mode(SPI_MODE_3)
		.build();
	spi.configure(&opts)?;

	Ok(spi)
}

fn main() {
	let spi = build_spi().unwrap();
	let mut psp = PlayStationPort::new(spi);

    loop {
        let buttons = psp.read_buttons();
        println!("Start? {}", buttons.start());
    }
}
