#![feature(duration_from_micros)]

extern crate linux_embedded_hal as linux_hal;
extern crate embedded_hal;
extern crate bit_reverse;

use bit_reverse::ParallelReverse;
use std::io;
use std::{thread, time};
use linux_hal::spidev::{Spidev, SpidevOptions, SpidevTransfer, SPI_MODE_3};

fn build_spi() -> io::Result<Spidev> {
	let mut spi = Spidev::open("/dev/spidev32766.0")?;
	let opts = SpidevOptions::new()
		.bits_per_word(8)
		.max_speed_hz(50_000)
		.mode(SPI_MODE_3)
		//.lsb_first(true)  // Hardware may not support this
		.build();
	spi.configure(&opts)?;

	Ok(spi)
}

fn main() {
	let spi = build_spi().unwrap();
	let mut command = [0u8; 21];
	let mut buffer = [0u8; 21];

	command[0] = 0x01;
	command[1] = 0x42;
	command[2] = 0x00;

	for item in command.iter_mut() {
		*item = item.swap_bits();
	}

	let mut now = time::Instant::now();
	let sleep_duration = time::Duration::from_micros(30_000);
	let sample_duration = time::Duration::from_secs(1);
	let mut count = 0;
	let mut failure = 0;
	let mut rate = String::new();

	loop {
		thread::sleep(sleep_duration);	

		{
			let mut transfer = SpidevTransfer::read_write(&command, &mut buffer);
			spi.transfer(&mut transfer).unwrap();
		}

		if now.elapsed() > sample_duration {
			now = time::Instant::now();
			rate = format!("{0:04}/{1:04}", count, failure);
			count = 0;
			failure = 0;
		}
		println!("");
		print!("Rate: ({}) - ", rate);

		for item in buffer.iter() {
			print!("{:02x} ", item.swap_bits());
		}

		if buffer[1] == 0xff {
			failure += 1;
		} else {
			count += 1;
		}
	}
}
