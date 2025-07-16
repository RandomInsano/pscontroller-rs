extern crate bit_reverse;
extern crate linux_embedded_hal as linux_hal;
extern crate pscontroller_rs;

use linux_hal::spidev::{SpidevOptions, SPI_MODE_3};
use linux_hal::Pin;
use linux_hal::Spidev;
use std::io;

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

fn dump_hex(buffer: &[u8]) {
    for byte in buffer.iter() {
        print!("{:02x} ", byte);
    }
    println!();
}

fn main() {
    let spi = build_spi().unwrap();
    let mut psp = PlayStationPort::new(spi, None::<Pin>);

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

    loop {
        let controller = match psp.read_input(None) {
            Err(_) => {
                print!("\rError reading controller");
                continue;
            }
            Ok(x) => x,
        };

        match controller {
            Device::None => {
                println!("Missing.");
            }
            Device::Mouse(x) => {
                println!(
                    "Mouse: X:{:02}, Y{:02}, L:{}, R:{}",
                    x.x,
                    x.y,
                    x.buttons.left(),
                    x.buttons.right()
                );
            }
            Device::Classic(x) => {
                println!(
                    "Classic - Start? {0}, Square? {1}",
                    x.buttons.start(),
                    x.buttons.square()
                );

                if x.buttons.start() && x.buttons.select() {
                    psp.enable_pressure().unwrap();
                }
            }
            Device::AnalogJoystick(x) => {
                println!(
                    "Analog - Start? {0} - R:{1:02x},{2:02x}, L:{3:02x},{4:02x}",
                    x.buttons.start(),
                    x.rx,
                    x.ry,
                    x.lx,
                    x.ly
                );
            }
            Device::DualShock(x) => {
                println!(
                    "DualShock - Start? {0} - R:{1:02x},{2:02x}, L:{3:02x},{4:02x}",
                    x.buttons.start(),
                    x.rx,
                    x.ry,
                    x.lx,
                    x.ly
                );
            }
            Device::DualShock2(x) => {
                println!(
                    "DualShock2 - Start? {0} - R:{1:02x},{2:02x} - X Pressure:{3:02x}",
                    x.buttons.start(),
                    x.rx,
                    x.ry,
                    x.pressures[6]
                );
            }
            Device::JogCon(x) => {
                println!(
                    "JogCon - Buttons: {0:08b}, Wheel: {1}",
                    x.buttons.bits(),
                    x.jog_position()
                )
            }
            Device::NegCon(x) => {
                println!(
                    "NegCon- Buttons: {0:08b}, Twist: {1}, I:  {2}",
                    x.buttons.bits(),
                    x.twist,
                    x.switchi
                )
            }
            Device::GunCon(x) => {
                print!(
                    "\rGunCon - Trigger: {0}, A: {3}, B:{4}, X:{1} Y:{2}                    ",
                    x.buttons.trigger(),
                    x.x(),
                    x.y(),
                    x.buttons.a(),
                    x.buttons.b()
                )
            }
            Device::ConfigurationMode => {
                println!("Somehow we got stuck where we shouldn't be");
            }
            _ => println!("Unimplemented"),
        }
    }
}
