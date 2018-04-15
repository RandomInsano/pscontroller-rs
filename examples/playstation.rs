extern crate linux_embedded_hal as linux_hal;
extern crate bit_reverse;
extern crate pscontroller_rs;

use std::io;
use linux_hal::Spidev;
use linux_hal::spidev::{SpidevOptions, SPI_MODE_3};
use linux_hal::Pin;

use pscontroller_rs::{PlayStationPort, Device};

// Specific to the host device used on Linux, you'll have to change the following
// parameters depending on your board and also export and allow writing to the GPIO
const SPI_ENABLE_PIN: u64 = 1020; // XIO-7 on the NTC CHIP
const SPI_DEVICE: &str = "/dev/spidev32766.0"; // Needs a device-tree overlay
const SPI_SPEED: u32 = 50_000; // Due to a bug, 50KHz is the best the CHIP can do semi-reliably

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

fn dump_hex(buffer: &[u8]) {
    for byte in buffer.iter() {
        print!("{:02x} ", byte);
    }
    println!();
}

fn main() {
    let spi = build_spi().unwrap();
    let enable_pin = Pin::new(SPI_ENABLE_PIN);    
    let mut psp = PlayStationPort::new(spi, enable_pin);

    psp.enable_pressure().unwrap();

    // Constants seem to be the same across many different controllers,
    // but I'd like to build a respository one day to see if there are
    // reliable ways of detecting them.
    let config = psp.read_config().unwrap();
    print!("Status:    ");
    dump_hex(&config.status);
    print!("Const 1.1: ");
    dump_hex(&config.const1a);
    print!("Const 1.2: ");
    dump_hex(&config.const1b);
    print!("Const 2:   ");
    dump_hex(&config.const2);
    print!("Const 3.1: ");
    dump_hex(&config.const3a);
    print!("Const 3.2: ");
    dump_hex(&config.const3a);

    println!("Press [start] in standard mode to start polling");
    loop {
        let controller = match psp.read_input() {
            Err(_) => {
                print!("\rError reading from port");
                continue;
            },
            Ok(x) => x,
        };

        match controller {
            Device::Classic(x) => {
                if x.buttons.start() {
                    break;
                }
            },
            Device::DualShock2(x) => {
                if x.buttons.start() {
                    break;
                }
            }
            _ => {}
        }
    }

    loop {
        let controller = match psp.read_input() {
            Err(_) => {
                print!("\rError reading controller");
                continue;
            },
            Ok(x) => x,
        };

        match controller {
            Device::None => {
                println!("Missing.");
            },
            Device::Classic(x) => {
                println!("Start? {0}, Square? {1}",
                    x.buttons.start(),
                    x.buttons.square());

                if x.buttons.start() && x.buttons.select() {
                    psp.enable_pressure().unwrap();
                }
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
            Device::GuitarHero(x) => {
                println!("Buttons: {0:08b}, Whammy: {1}", x.buttons, x.whammy)
            }
            Device::ConfigurationMode => {
                println!("Somehow we got stuck where we shouldn't be");
            },
            _ => println!("Unimplemented"),
        }
    }
}
