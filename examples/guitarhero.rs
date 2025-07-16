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

// This will build the
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

        // We can't guess that we have a GuitarHero controller as it
        // appears nearly the same as a first-generation DualShock, except
        // for some buttons that are held down which isn't reliable
        unsafe {
            controller = controller_data.gh;
        }

        //        println!("G:{0:1b}, R:{0:1b}, Y:{0:1b}, B:{0:1b}, O:{0:1b}",
        println!(
            "G:{}\tR:{}\tY:{}\tB:{}\tO:{}\tSelect:{}",
            controller.buttons.fret_green(),
            controller.buttons.fret_red(),
            controller.buttons.fret_yellow(),
            controller.buttons.fret_blue(),
            controller.buttons.fret_orange(),
            controller.buttons.select()
        );
        println!(
            "SU:{}\tSD:{}\tStar power:{}\tWhammy:{}",
            controller.buttons.strum_up(),
            controller.buttons.strum_down(),
            controller.buttons.star_power(),
            controller.whammy
        );
        println!();
    }
}
