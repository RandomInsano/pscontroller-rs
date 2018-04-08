#![feature(untagged_unions)]

#[macro_use]
extern crate bitflags;

extern crate bit_reverse;
extern crate embedded_hal as hal;

use bit_reverse::ParallelReverse;
use hal::blocking::spi;

const PS_SELECT: u16 = 0x0001;
const PS_L3: u16 = 0x0002;
const PS_R3: u16 = 0x0004;
const PS_START: u16 = 0x0008;

const PS_UP: u16 = 0x0010;
const PS_RIGHT: u16 = 0x0020;
const PS_DOWN: u16 = 0x0040;
const PS_LEFT: u16 = 0x0080;

const PS_L2: u16 = 0x0100;
const PS_R2: u16 = 0x0200;
const PS_L1: u16 = 0x0400;
const PS_R1: u16 = 0x0800;

const PS_TRIANGLE: u16 = 0x1000;
const PS_CIRCLE: u16 = 0x2000;
const PS_CROSS: u16 = 0x4000;
const PS_SQUARE: u16 = 0x8000;

/// Controller missing
const CONTROLLER_NOT_PRESENT: u8 = 0xff;
/// Original controller, SCPH-1080
const CONTROLLER_CLASSIC: u8 = 0xc1;
/// DualShock in Digital mode
const CONTROLLER_DUALSHOCK_DIGITAL: u8 = 0x41;
/// DualShock
const CONTROLLER_DUALSHOCK_ANALOG: u8 = 0x73;
/// DuakShock 2
const CONTROLLER_DUALSHOCK_PRESSURE: u8 = 0x79;

/// Command to poll buttons
const CMD_POLL: &[u8] = &[0x01, 0x42, 0x00];

#[repr(C)]
union ControllerData {
    data: [u8; 18],
    classic: Classic,
    ds: DualShock,
    ds2: DualShock2,
}

/// The digital buttons of the gamepad
#[repr(C)]
pub struct GamepadButtons {
    data: u16,
}

/// Error
#[derive(Debug)]
pub enum Error<E> {
    /// Late collision
    LateCollision,
    /// SPI error
    Spi(E),
}

impl<E> From<E> for Error<E> {
    fn from(e: E) -> Self {
        Error::Spi(e)
    }
}

/// Gamepad buttons
impl GamepadButtons {
    // 2018: I do wish bit packing was finally a thing...
    // Gamepad buttons are active low, so that's why we're
    // comparing them to zero instead of not zero

    pub fn select(&self) -> bool {
        self.data & PS_SELECT == 0
    }

    pub fn l3(&self) -> bool {
        self.data & PS_L3 == 0
    }

    pub fn r3(&self) -> bool {
        self.data & PS_R3 == 0
    }

    pub fn start(&self) -> bool {
        self.data & PS_START == 0
    }

    pub fn up(&self) -> bool {
        self.data & PS_UP == 0
    }

    pub fn right(&self) -> bool {
        self.data & PS_RIGHT == 0
    }

    pub fn down(&self) -> bool {
        self.data & PS_DOWN == 0
    }

    pub fn left(&self) -> bool {
        self.data & PS_LEFT == 0
    }

    pub fn l2(&self) -> bool {
        self.data & PS_L2 == 0
    }

    pub fn r2(&self) -> bool {
        self.data & PS_R2 == 0
    }

    pub fn l1(&self) -> bool {
        self.data & PS_L1 == 0
    }

    pub fn r1(&self) -> bool {
        self.data & PS_R1 == 0
    }

    pub fn triangle(&self) -> bool {
        self.data & PS_TRIANGLE == 0
    }

    pub fn circle(&self) -> bool {
        self.data & PS_CIRCLE == 0
    }

    pub fn cross(&self) -> bool {
        self.data & PS_CROSS == 0
    }

    pub fn square(&self) -> bool {
        self.data & PS_SQUARE == 0
    }
}

#[repr(C)]
pub struct DualShock2 {
    pub buttons: GamepadButtons,

    pub rx: u8,
    pub ry: u8,
    pub lx: u8,
    pub ly: u8,

    /// List of possible pressure readings from the buttons
    /// Note that these are configurable length
    pub pressures: [u8; 8],
}

#[repr(C)]
pub struct DualShock {
    pub buttons: GamepadButtons,

    pub rx: u8,
    pub ry: u8,
    pub lx: u8,
    pub ly: u8,    
}

#[repr(C)]
pub struct Classic {
    pub buttons: GamepadButtons,
}

pub enum Device {
    None,
    Unknown,
    Classic(Classic),
    DualShock(DualShock),
    DualShock2(DualShock2),
}

/// The main event! Create a port using an SPI bus and start commanding
/// controllers!
pub struct PlayStationPort<SPI> {
    dev: SPI,
}

impl<E, SPI> PlayStationPort<SPI>
where
    SPI: spi::Transfer<u8, Error = E> {

    pub fn new(spi: SPI) -> Self {
        Self {
            dev: spi,
        }
    }

    fn flip(bytes: &mut [u8]) {
        for byte in bytes.iter_mut() {
		    *byte = byte.swap_bits();
	    }
    }

    // Needed because `copy_from_slice` doesn't work on different
    // sized arrays for some silly reason
    fn byte_copy(from: &[u8], to: &mut [u8]) {
        assert!(from.len() <= to.len());

        for i in 0 .. from.len() {
            to[i] = from[i];
        }
    }

    pub fn send_command(&mut self, command: &mut [u8]) -> Result<(), E> {
        Self::flip(command);
        self.dev.transfer(command)?;
        Self::flip(command);

        Ok(())
    }

    // TODO: Return error types
    pub fn read_buttons(&mut self) -> Device {
        let mut buffer = [0u8; 21];
        let mut data = [0u8; 18];
        Self::byte_copy(CMD_POLL, &mut buffer);
        self.send_command(&mut buffer);

        data.copy_from_slice(&buffer[3 .. 21]);

        let controller = ControllerData { data: data };

        unsafe {
            match buffer[1] {
                CONTROLLER_NOT_PRESENT => Device::None,
                CONTROLLER_CLASSIC => Device::Classic(controller.classic),
                CONTROLLER_DUALSHOCK_DIGITAL => Device::Classic(controller.classic),                
                CONTROLLER_DUALSHOCK_ANALOG => Device::DualShock(controller.ds),
                CONTROLLER_DUALSHOCK_PRESSURE => Device::DualShock2(controller.ds2),
                _ => Device::Unknown,
            }
        }
    }
}

mod tests {
    #[test]
    fn union_test() {
        // Again, buttons are active low, hence 'fe' and '7f'
        let controller = ControllerData {
            data: [
                0xfe,
                0x7f,
                0x00,
                0x00,
                0x00,
                0xff
            ],
        };

        unsafe {
            assert!(controller.ds.buttons.select() == true);
            assert!(controller.ds.buttons.square() == true);
            assert!(controller.ds.lx == 0);
            assert!(controller.ds.ly == 255);
        }
    }
}