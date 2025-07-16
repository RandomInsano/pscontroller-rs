extern crate linux_embedded_hal as linux_hal;
extern crate pscontroller_rs;

use std::{io, thread, time};

use linux_hal::spidev::{SpidevOptions, SPI_MODE_3};
use linux_hal::Pin;
use linux_hal::Spidev;

use pscontroller_rs::{Device, PlayStationPort};

// Specific to the host device used on Linux, you'll have to change the following
// parameters depending on your board and also export and allow writing to the GPIO
const SPI_DEVICE: &str = "/dev/spidev0.0";
const SPI_SPEED: u32 = 100_000;

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
    let sleep_duration = time::Duration::from_millis(10);

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    loop {
        let controller = psp.read_input(None).unwrap();

        match controller {
            Device::None => {
                println!("Missing.");
            }
            Device::Mouse(mouse) => {
                x += mouse.x as i32;
                y += mouse.y as i32;

                println!(
                    "Mouse: X:{:04}, Y{:04}, L:{}, R:{}",
                    x,
                    y,
                    mouse.buttons.left(),
                    mouse.buttons.right()
                );

                // Sleep is required for polling on my cheap third party mouse
                thread::sleep(sleep_duration);
            }
            _ => println!("Not supported for this example"),
        }
    }
}
