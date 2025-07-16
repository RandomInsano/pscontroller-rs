extern crate bit_reverse;
extern crate linux_embedded_hal as linux_hal;
extern crate pscontroller_rs;

use linux_hal::spidev::{SpidevOptions, SPI_MODE_3};
use linux_hal::Pin;
use linux_hal::Spidev;
use std::io;

use pscontroller_rs::PlayStationPort;

// Specific to the host device used on Linux, you'll have to change the following
// parameters depending on your board and also export and allow writing to the GPIO
const SPI_DEVICE: &str = "/dev/spidev0.0";
const SPI_SPEED: u32 = 10_000;

// This will build the SPI device communication for us
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

    let mut controller;

    psp.enable_pressure().unwrap();

    loop {
        let controller_data = match psp.read_raw(None) {
            Err(_) => {
                print!("\rError reading controller");
                continue;
            }
            Ok(x) => x,
        };

        // It's not possible to tell the baton apart from a NeGcon or other
        // devices that respond as type "0x2", so we have to force it for now.
        unsafe {
            controller = controller_data.b;
        }

        println!(
            "\rA:{}, B:{} - Z: {:03} X: {:03} Y: {:03} A: {:03}",
            controller.buttons.a(),
            controller.buttons.b(),
            controller.z,
            controller.x,
            controller.y,
            controller.a
        );
    }
}
