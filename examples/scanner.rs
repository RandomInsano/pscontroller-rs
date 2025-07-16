extern crate embedded_hal;
extern crate linux_embedded_hal as linux_hal;
extern crate pscontroller_rs;

use linux_hal::spidev::{SpidevOptions, SPI_MODE_3};
use linux_hal::Pin;
use linux_hal::Spidev;
use std::io;
use std::{thread, time};

use pscontroller_rs::{MultitapPort, PlayStationPort};

const CMD_ENTER_ESCAPE_MODE: &[u8] = &[0x00, 0x43, 0x00, 0x01, 0x00];
const CMD_EXIT_ESCAPE_MODE: &[u8] = &[0x00, 0x43, 0x00, 0x00, 0x00];
const MULTITAP_LIST: [MultitapPort; 4] = [
    MultitapPort::A,
    MultitapPort::B,
    MultitapPort::C,
    MultitapPort::D,
];

const SPI_DEVICE: &str = "/dev/spidev0.0";
const SPI_SPEED: u32 = 50_000;

const SCAN_RESPONSE_WIDTH: u8 = 10;
const SAMPLE_PAUSE: u64 = 0_000;
const USE_MULTITAP: bool = false;

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
    let mut command = [0u8; SCAN_RESPONSE_WIDTH as usize];
    let mut buffer = [0u8; SCAN_RESPONSE_WIDTH as usize];
    let mut _dummy = [0u8; SCAN_RESPONSE_WIDTH as usize];

    let sleep_duration = time::Duration::from_micros(SAMPLE_PAUSE);

    psp.enable_pressure().unwrap();

    for i in MULTITAP_LIST.iter() {
        println!("                                                  ");
        println!(
            "Scanning port address {0:02x}                     ",
            i.clone() as u8
        );
        println!("==================================================");

        // I had trouble with both something called `type ascription` and conflicting
        // datatypes when I tried to dynamically generate fixed-sized arrays of multitap
        // ports, so I'm doing things the lazy way and bailing out! :D
        if USE_MULTITAP && *i == MultitapPort::B {
            return;
        }

        for k in 0..2 {
            let escape = k == 1;

            if escape {
                println!("Escape Commands:                        ");
            } else {
                println!("Regular Commands:                         ");
            }

            for j in 0..=0xff {
                psp.set_multitap_port(i.clone());
                command[1] = j;

                if escape {
                    psp.send_command(CMD_ENTER_ESCAPE_MODE, &mut _dummy)
                        .unwrap();
                    thread::sleep(sleep_duration);
                }

                psp.send_command(&command, &mut buffer).unwrap();

                if escape {
                    psp.send_command(CMD_EXIT_ESCAPE_MODE, &mut _dummy).unwrap();
                }

                print!("Command {:02x}: ", j);

                let mut found = false;
                for k in 2..buffer.len() {
                    if buffer[k] != 0xff {
                        found = true;
                    }
                }
                if !found {
                    print!("\r");
                    continue;
                }

                for item in buffer.iter() {
                    print!("{:02x} ", item);
                }

                println!("");
            }
        }

        println!("                                         ");
    }
}
