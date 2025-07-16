extern crate bit_reverse;
extern crate linux_embedded_hal as linux_hal;
extern crate pscontroller_rs;

use linux_hal::spidev::{SpidevOptions, SPI_MODE_3};
use linux_hal::Pin;
use linux_hal::Spidev;
use std::io;

use pscontroller_rs::{classic::GamepadButtons, dualshock::ControlDS, Device, PlayStationPort};

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

fn set_motors(buttons: &GamepadButtons, small: &mut bool, big: &mut u8) {
    if buttons.cross() {
        *small = true;
    } else {
        *small = false;
    }

    if buttons.down() {
        *big = 255 / 3;
    } else if buttons.left() {
        *big = 255 / 2;
    } else if buttons.up() {
        *big = 255 / 1;
    } else {
        *big = 0;
    }
}

fn main() {
    let spi = build_spi().unwrap();
    let mut psp = PlayStationPort::new(spi, None::<Pin>);
    let mut control_ds = ControlDS::new(false, 0);

    let mut big: u8 = 0;
    let mut small: bool = false;

    psp.enable_pressure().unwrap();

    loop {
        control_ds.little = small;
        control_ds.big = big;

        let controller = match psp.read_input(Some(&control_ds)) {
            Err(_) => {
                print!("\rError reading controller");
                continue;
            }
            Ok(x) => x,
        };

        match controller {
            Device::DualShock(x) | Device::AnalogJoystick(x) => {
                println!(
                    "DualShock:   Start? {0} - R:{1:02x},{2:02x}, L:{3:02x},{4:02x}",
                    x.buttons.start(),
                    x.rx,
                    x.ry,
                    x.lx,
                    x.ly
                );

                set_motors(&x.buttons, &mut small, &mut big);
            }
            Device::DualShock2(x) => {
                println!(
                    "DualShock2:  Start? {0} - R:{1:02x},{2:02x} - X Pressure:{3:02x}",
                    x.buttons.start(),
                    x.rx,
                    x.ry,
                    x.pressures[6]
                );

                set_motors(&x.buttons, &mut small, &mut big);
            }
            Device::None => println!("Please plug in a controller"),
            _ => println!("This example doesn't support the current controller"),
        }
    }
}
