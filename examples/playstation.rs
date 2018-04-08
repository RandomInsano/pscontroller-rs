extern crate linux_embedded_hal as linux_hal;
extern crate bit_reverse;
extern crate pscontroller_rs;

use std::io;
use linux_hal::Spidev;
use linux_hal::spidev::{SpidevOptions, SPI_MODE_3};

use pscontroller_rs::{PlayStationPort, Device};

fn build_spi() -> io::Result<Spidev> {
	let mut spi = Spidev::open("/dev/spidev32766.0")?;
	let opts = SpidevOptions::new()
		.bits_per_word(8)
		.max_speed_hz(20_000)
		.mode(SPI_MODE_3)
		.build();
	spi.configure(&opts)?;

	Ok(spi)
}

fn main() {
    let spi = build_spi().unwrap();
    let mut psp = PlayStationPort::new(spi);

    psp.enable_pressure();

    loop {
        let controller = psp.read_buttons();

        match controller {
            Device::None => {
                println!("Missing.");
            },
            Device::Classic(x) => {
                println!("Start? {0}, Square? {1}",
                    x.buttons.start(),
                    x.buttons.square());
            },
            Device::DualShock(x) => {
                println!("Start? {0} - R:{1:02x},{2:02x}, L:{3:02x},{4:02x}", 
                    x.buttons.start(),
                    x.rx,
                    x.ry,
                    x.lx,
                    x.ly);
            },
            Device::DualShock2(x) => {
                println!("Start? {0} - R:{1:02x},{2:02x} - X Pressure:{3:02x}", 
                    x.buttons.start(),
                    x.rx,
                    x.ry,
                    x.pressures[6]);
            },            
            _ => println!("Unimplemented"),
        }
    }
}
