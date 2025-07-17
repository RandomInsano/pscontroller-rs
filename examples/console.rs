extern crate embedded_hal;
extern crate linux_embedded_hal as linux_hal;
extern crate pscontroller_rs;

use linux_hal::spidev::{SpiModeFlags, SpidevOptions};
use linux_hal::{CdevPin, SpidevBus};
use std::io;
use std::{thread, time};

use pscontroller_rs::PlayStationPort;

// Specific to the host device used on Linux, you'll have to change the following
// parameters depending on your board and also export and allow writing to the GPIO
const SPI_DEVICE: &str = "/dev/spidev0.0";
const SPI_SPEED: u32 = 100_000;
// If you need to use an alternate pin for cable select, uncomment the relevant bits
// and pass the pin into psp's new() function.
//const SPI_ENABLE_PIN: u32 = 4;

fn build_spi() -> Result<SpidevBus, Box<dyn std::error::Error>> {
    let mut spi = SpidevBus::open(SPI_DEVICE)?;
    let opts = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(SPI_SPEED)
        .mode(SpiModeFlags::SPI_MODE_3)
        .build();
    spi.configure(&opts)?;

    Ok(spi)
}

fn main() {
    let spi = build_spi().unwrap();
    // Example of using GPIO pin for chip select:
    //const GPIO_CHIP: &str = "/dev/gpiochip0";
    //let mut chip = Chip::new(GPIO_CHIP).unwrap();
    //let enable_pin = CdevPin::new(chip.get_line(SPI_ENABLE_PIN).unwrap()
    //    .request(LineRequestFlags::OUTPUT, 1, "pscontroller").unwrap()).unwrap();
    //let mut psp = PlayStationPort::new(spi, Some(enable_pin));
    let mut psp: PlayStationPort<_, CdevPin> = PlayStationPort::new(spi, None);
    let mut command = [0u8; 32];
    let mut buffer = [0u8; 32];

    command[1] = 0x42;

    let mut now = time::Instant::now();
    let sleep_duration = time::Duration::from_micros(20_000);
    let sample_duration = time::Duration::from_secs(1);
    let mut count = 0;
    let mut failure = 0;
    let mut rate = String::new();

    psp.enable_pressure().unwrap();

    loop {
        thread::sleep(sleep_duration);

        psp.send_command(&command, &mut buffer).unwrap();

        if now.elapsed() > sample_duration {
            now = time::Instant::now();
            rate = format!("{0:04}/{1:04}", count, failure);
            count = 0;
            failure = 0;
        }
        println!("");
        print!("Rate: ({}) - ", rate);

        // Print the three byte header and X * 16bit message
        let mut c = 3 + (buffer[1] & 0xF) * 2;
        for item in buffer.iter() {
            // Only print the number of bytes the controller claims exists
            if c == 0 {
                break;
            }

            print!("{:02x} ", item);
            c -= 1;
        }

        if buffer[1] == 0xff {
            failure += 1;
        } else {
            count += 1;
        }
    }
}
