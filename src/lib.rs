//! Playstation Controller driver for Rust's [Embedded Hardware Abstraction Layer](https://github.com/japaric/embedded-hal)
//! ============================
//! 
//! The original PlayStation and most of its peripherals are 20+ years old at this point,
//! but they're easy to interface with for fun projects, and wireless variants are easy
//! to come by while being [pretty cheap](https://www.amazon.ca/s/?ie=UTF8&keywords=ps2+wireless+controller)
//! ($28 Canadian for two as of this writing).
//! 
//! The current state of this library is such that it's fairly naïve to which controller
//! is plugged in, and will make a guess based on the device type (1 - 15) and
//! how many 16bit words are being returned. For those who know the protocol, this library
//! reads the second byte from the poll command and wraps the response in a struct based
//! on that so it's lightweight on memory
//! 
//! Efficiencies can be made here, and things will likely improve, but the darn thing is
//! useful now so let's start using it! If you find things to fix, please make an issue.
//! 
//! Hardware
//! -----------------------
//! 
//! Because the PlayStation could have up to four devices sharing the same SPI bus (two
//! controllers and two memory cards), they made the data out (MISO) pin open drain so you'll
//! need to add your own resistor. In my testing with a voltage divider on a PlayStation 2,
//! the value is around 220 - 500 ohms. I'm not sure if the controller jack assembly
//! contains one of these yet, so you should double check with a multimeter before plugging
//! anything in.

#![feature(untagged_unions)]
#![no_std]
#![deny(missing_docs)]
#![deny(warnings)]
#![feature(unsize)]

extern crate bit_reverse;
extern crate bitflags;
extern crate embedded_hal as hal;

use bit_reverse::ParallelReverse;
use hal::blocking::spi;
use hal::digital::OutputPin;

/// The maximum length of a message from a controller
const MESSAGE_MAX_LENGTH: usize = 32;
/// Acknoweldgement byte for header commnad
//const ACK_BYTE: u8 = 0x5a;
/// Length of the command header
const HEADER_LEN: usize = 3;

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
/// JogCon
const CONTROLLER_JOGCON: u8 = 0xe3;
/// NegCon
const CONTROLLER_NEGCON: u8 = 0x23;
/// Configuration Mode
const CONTROLLER_CONFIGURATION: u8 = 0xf3;

/// Command to poll buttons
const CMD_POLL: &[u8] = &[0x42, 0x00];
/// Command to enter escape mode
const CMD_ENTER_ESCAPE_MODE: &[u8] = &[0x43, 0x00, 0x01, 0x00];
/// Command to exit escape mode
const CMD_EXIT_ESCAPE_MODE: &[u8] = &[0x43, 0x00, 0x00, 0x00];
/// Command to set response format. Right now asks for all data
const CMD_RESPONSE_FORMAT: &[u8] = &[0x4F, 0x00, 0xFF, 0xFF, 0x03, 0x00, 0x00, 0x00];
/// Command to initialize / customize pressure
const CMD_INIT_PRESSURE: &[u8] = &[0x40, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00];
/// Command to set major mode (DualShock = 1 / Digital = 0)
const CMD_SET_MODE: &[u8] = &[0x44, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00];
/// Command to read extended status
const CMD_READ_STATUS: &[u8] = &[0x45, 0x00, 0x5a, 0x5a, 0x5a, 0x5a, 0x5a, 0x5a];
/// Command to read constant 1 at address 00
const CMD_READ_CONST1A: &[u8] = &[0x46, 0x00, 0x00, 0x5a, 0x5a, 0x5a, 0x5a, 0x5a];
/// Command to read constant 1 at address 01
const CMD_READ_CONST1B: &[u8] = &[0x46, 0x00, 0x01, 0x5a, 0x5a, 0x5a, 0x5a, 0x5a];
/// Command to read constant 2 at address 00
const CMD_READ_CONST2: &[u8] = &[0x47, 0x00, 0x00, 0x5a, 0x5a, 0x5a, 0x5a, 0x5a];
/// Command to read constant 3 at address 00
const CMD_READ_CONST3A: &[u8] = &[0x4C, 0x00, 0x00, 0x5a, 0x5a, 0x5a, 0x5a, 0x5a];
/// Command to read constant 3 at address 01
const CMD_READ_CONST3B: &[u8] = &[0x4C, 0x00, 0x01, 0x5a, 0x5a, 0x5a, 0x5a, 0x5a];
/// Command to enable JogCon motor
const CMD_MOTOR_JOGCON: &[u8] = &[0x4D, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff]; 

#[repr(C)]
/// The poll command returns a series of bytes. This union allows us to interact with
/// those bytes with some functions. It's nice sugar that allows us to use the data
/// in an obvious way without needing to copy the bytes around
union ControllerData {
    data: [u8; MESSAGE_MAX_LENGTH],
    classic: Classic,
    ds: DualShock,
    ds2: DualShock2,
    gh: GuitarHero,
    jc: JogCon,
    nc: NegCon,
}

/// What we want the JogCon's wheel to do after we
/// poll it
pub enum JogControl {
    /// Stop the motor
    Stop = 0x00,
    /// Hold the wheel in position (and return it if it moves)
    Hold = 0x30,
    /// Start turning the wheel left
    Left = 0x20,
    /// Start turning the wheel right
    Right = 0x10,
    /// Unknown 1
    Unknown1 = 0x80,
    /// Unknown 2
    Unknown2 = 0xb0,
    /// Unknown 3
    Unknown3 = 0xc0,
}

/// What state the JogCon's wheel was in last poll
pub enum JogState {
    /// The wheel was turned left
    TurnedLeft,
    /// The wheel was turned right
    TurnedRight,
    /// The wheel met its maximum recordable distance
    AtMaximum
}

/// The active port to set on the Multitap
#[derive(Clone)]
pub enum MultitapPort {
    /// The first port on the multi-tap and also the port when no tap
    /// is present
    A = 0x01,
    /// The second port on the multi-top
    B = 0x02,
    /// The third port on the multi-top
    C = 0x03,
    /// The fourth port on the multi-top
    D = 0x04,
    /// This may be for the multitap itself
    X = 0xff,
}

/// The digital buttons of the gamepad
#[repr(C)]
#[derive(Clone)]
pub struct GamepadButtons {
    data: u16,
}

/// The digital buttons of the Namco NegCon
#[repr(C)]
#[derive(Clone)]
pub struct NegconButtons {
    data: u16,
}


/// Errors that can arrise from trying to communicate with the controller
pub enum Error<E> {
    /// Late collision
    LateCollision,
    /// Something responded badly
    BadResponse,
    /// SPI error
    Spi(E),
}

impl<E> From<E> for Error<E> {
    fn from(e: E) -> Self {
        Error::Spi(e)
    }
}

/// Many controllers have the same set of buttons (Square, Circle, L3, R1, etc).
/// The devices that do have these buttons implement this trait. Depite the original
/// Controller not having L3 and R3, they are brought out regardless and just considered
/// unpressable.
pub trait HasStandardButtons {
    /// This does require a clone operation of the bytes inside the controller.
    /// To save yourself the copy, you can access the button data directly via `buttons`
    fn buttons(&self) -> GamepadButtons;
}

/// A collection of helper functions to take the button bitfield and make them more
/// ergonomic to use.
impl GamepadButtons {
    // Gamepad buttons are active low, so that's why we're comparing them to zero

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

    /// A button on the controller
    pub fn select(&self) -> bool {
        self.data & Self::PS_SELECT == 0
    }

    /// A button on the controller
    pub fn l3(&self) -> bool {
        self.data & Self::PS_L3 == 0
    }

    /// A button on the controller
    pub fn r3(&self) -> bool {
        self.data & Self::PS_R3 == 0
    }

    /// A button on the controller
    pub fn start(&self) -> bool {
        self.data & Self::PS_START == 0
    }

    /// A button on the controller
    pub fn up(&self) -> bool {
        self.data & Self::PS_UP == 0
    }

    /// A button on the controller
    pub fn right(&self) -> bool {
        self.data & Self::PS_RIGHT == 0
    }

    /// A button on the controller
    pub fn down(&self) -> bool {
        self.data & Self::PS_DOWN == 0
    }

    /// A button on the controller
    pub fn left(&self) -> bool {
        self.data & Self::PS_LEFT == 0
    }

    /// A button on the controller
    pub fn l2(&self) -> bool {
        self.data & Self::PS_L2 == 0
    }

    /// A button on the controller
    pub fn r2(&self) -> bool {
        self.data & Self::PS_R2 == 0
    }

    /// A button on the controller
    pub fn l1(&self) -> bool {
        self.data & Self::PS_L1 == 0
    }

    /// A button on the controller
    pub fn r1(&self) -> bool {
        self.data & Self::PS_R1 == 0
    }

    /// A button on the controller
    pub fn triangle(&self) -> bool {
        self.data & Self::PS_TRIANGLE == 0
    }

    /// A button on the controller
    pub fn circle(&self) -> bool {
        self.data & Self::PS_CIRCLE == 0
    }

    /// A button on the controller
    pub fn cross(&self) -> bool {
        self.data & Self::PS_CROSS == 0
    }

    /// A button on the controller
    pub fn square(&self) -> bool {
        self.data & Self::PS_SQUARE == 0
    }

    /// The raw value of the buttons on the controller. Useful for
    /// aggregate functions
    pub fn bits(&self) -> u16 {
        self.data
    }
}

/// The NegCon's version of `GamepadButtons`
impl NegconButtons {
    // Gamepad buttons are active low, so that's why we're comparing them to zero

    const NC_SELECT: u16 = 0x0001;
    const NC_START: u16 = 0x0008;
    const NC_UP: u16 = 0x0010;
    const NC_RIGHT: u16 = 0x0020;
    const NC_DOWN: u16 = 0x0040;
    const NC_LEFT: u16 = 0x0080;

    const NC_R: u16 = 0x0800;
    const NC_B: u16 = 0x1000;
    const NC_A: u16 = 0x2000;


    /// A button on the controller
    pub fn select(&self) -> bool {
        self.data & Self::NC_SELECT == 0
    }

    /// A button on the controller
    pub fn start(&self) -> bool {
        self.data & Self::NC_START == 0
    }

    /// A button on the controller
    pub fn up(&self) -> bool {
        self.data & Self::NC_UP == 0
    }

    /// A button on the controller
    pub fn right(&self) -> bool {
        self.data & Self::NC_RIGHT == 0
    }

    /// A button on the controller
    pub fn down(&self) -> bool {
        self.data & Self::NC_DOWN == 0
    }

    /// A button on the controller
    pub fn left(&self) -> bool {
        self.data & Self::NC_LEFT == 0
    }

    /// A button on the controller
    pub fn r(&self) -> bool {
        self.data & Self::NC_R == 0
    }

    /// A button on the controller
    pub fn b(&self) -> bool {
        self.data & Self::NC_B == 0
    }

    /// A button on the controller
    pub fn a(&self) -> bool {
        self.data & Self::NC_A == 0
    }

    /// The raw value of the buttons on the controller. Useful for
    /// aggregate functions
    pub fn bits(&self) -> u16 {
        self.data
    }
}

/// Holds information about the controller's configuration and constants
#[derive(Default)]
pub struct ControllerConfiguration {
    /// The controller's current status and *perhaps* its generation
    pub status: [u8; 6],
    /// Unknown constant
    pub const1a: [u8; 5],
    /// Unknown constant
    pub const1b: [u8; 5],
    /// Unknown constant
    pub const2: [u8; 5],
    /// Unknown constant
    pub const3a: [u8; 5],
    /// Unknown constant
    pub const3b: [u8; 5],
}

#[repr(C)]
/// Represents the DualShock 2 controller
pub struct DualShock2 {
    /// Standard buttons (Cross, Circle, L3, Start, etc)
    pub buttons: GamepadButtons,

    /// Right analog stick, left and right
    pub rx: u8,
    /// Right analog stick, up and down
    pub ry: u8,
    /// Left analog stick, left and right
    pub lx: u8,
    /// Left analog stick, up and down
    pub ly: u8,

    /// List of possible pressure readings from the buttons
    /// Note that these are configurable length
    pub pressures: [u8; 8],
}

impl HasStandardButtons for DualShock2 {
    fn buttons(&self) -> GamepadButtons {
        self.buttons.clone()
    }
}

#[repr(C)]
/// Represents the DualShock 1 controller
pub struct DualShock {
    /// Standard buttons (Cross, Circle, L3, Start, etc)
    pub buttons: GamepadButtons,

    /// Right analog stick, left and right
    pub rx: u8,
    /// Right analog stick, up and down
    pub ry: u8,
    /// Left analog stick, left and right
    pub lx: u8,
    /// Left analog stick, up and down
    pub ly: u8,
}

impl HasStandardButtons for DualShock {
    fn buttons(&self) -> GamepadButtons {
        self.buttons.clone()
    }
}

#[repr(C)]
/// Represents the Namco NegCon controller
pub struct NegCon {
    /// The NegCon's weird buttons (A, B, R, etc)
    pub buttons: NegconButtons,

    /// The position of the twist (center = 0x80)
    pub twist: u8,

    /// Position of switch I
    pub switchi: u8,

    /// Position of switch II
    pub switchii: u8,

    /// Position of switch L
    pub switchl: u8
}

#[repr(C)]
/// Represents the Namco JogCon controller
pub struct JogCon {
    // TODO: Implement an endian-safe accessor for jog_position
    // TODO: Implement an enum accessor for jog_state

    /// Standard buttons (Cross, Circle, L3, Start, etc)
    pub buttons: GamepadButtons,

    /// The absolute position of the jog wheel
    pub jog_position: i16,

    /// What state is the jog wheel in
    pub jog_state: u8,
}

impl HasStandardButtons for JogCon {
    fn buttons(&self) -> GamepadButtons {
        self.buttons.clone()
    }
}

#[repr(C)]
/// Represents the classic Controller
pub struct Classic {
    /// Standard buttons (Cross, Circle, L3, Start)
    pub buttons: GamepadButtons,
}

impl HasStandardButtons for Classic {
    fn buttons(&self) -> GamepadButtons {
        self.buttons.clone()
    }
}

#[repr(C)]
/// Represents a Guitar Hero controller
pub struct GuitarHero {
    // TODO: Figure out GH's button layout (strum up/down, fret colours)

    /// The buttons, currently not mapped
    pub buttons: u16,

    // Lazily pad bytes
    padding: [u8; 3],

    /// The whammy bar's current position
    pub whammy: u8,
}

/// Possible devices that can be returned by the poll command to the controller.
/// Currently, we're relying both on the device type (high nybble) and the number
/// of 16bit words its returning (low nybble) to guess the device type.
/// 
/// While that's not ideal, I haven't found a better way to do this yet. It seems
/// as though the constants to define devices rarely change so for example, the
/// Guitar Hero controller reports in every way that it is a DualShock 1 controller.
/// Other devices, like the DVD remote don't even support escape mode so this
/// is the best I can do until we find a better way to get creative.
pub enum Device {
    /// If pulling the device type didn't work
    None,
    /// A new controller type we haven't seen before
    Unknown,
    /// The controller is waiting for configuration data. Users of the library should
    /// never need to see this state.
    ConfigurationMode,
    /// Original controller that shipped with the PlayStation. Only contains regular
    /// buttons. DualShock 1 and 2 can emulate this mode
    Classic(Classic),
    /// Controller with two analog sticks. This was the final controller style shipped with
    /// the original PlayStation
    DualShock(DualShock),
    /// Controller that shipped with the PlayStation 2. Has dual analog sticks but also pressure
    /// senstive buttons
    DualShock2(DualShock2),
    /// Controller that shipped with Guitar Hero
    GuitarHero(GuitarHero),
    /// The Namco JonCon
    JogCon(JogCon),
    /// The Namco NegCon
    NegCon(NegCon),
}

/// The main event! Create a port using an SPI bus and start commanding
/// controllers!
pub struct PlayStationPort<SPI, CS> {
    dev: SPI,
    select: Option<CS>,
    multitap_port: MultitapPort,
}

impl<E, SPI, CS> PlayStationPort<SPI, CS>
where
    SPI: spi::Transfer<u8, Error = E>,
    CS: OutputPin {

    /// Create a new device to talk over the PlayStation's controller
    /// port
    pub fn new(spi: SPI, mut select: Option<CS>) -> Self {
        if let Some(ref mut x) = select {
            x.set_high(); // Disable controller for now
        }

        Self {
            dev: spi,
            select: select,
            multitap_port: MultitapPort::A,
        }
    }

    fn flip(bytes: &mut [u8]) {
        for byte in bytes.iter_mut() {
		    *byte = byte.swap_bits();
	    }
    }

    /// Set the active port on the multi-tap. If no tap is being used, anything
    /// other than `A` will fail to return anything. Or so I assume! Setting this
    /// will mean any commands send will be directed towards that port indefinitely.
    pub fn set_multitap_port(&mut self, port: MultitapPort) {
        self.multitap_port = port;
    }

    /// Sends commands to the underlying hardware and provides responses
    pub fn send_command(&mut self, command: &[u8], result: &mut [u8]) -> Result<(), E> {
        // Pack in bytes for the command we'll be sending
        result[0] = self.multitap_port.clone() as u8;
        result[1 .. command.len() + 1].copy_from_slice(command);

        // Because not all hardware supports LSB mode for SPI, we flip
        // the bits ourselves
        Self::flip(result);

        if let Some(ref mut x) = self.select {
            x.set_low();
        }

        self.dev.transfer(result)?;

        if let Some(ref mut x) = self.select {
            x.set_high();
        }

        Self::flip(result);

        Ok(())
    }

    /// Configure the controller to set it to DualShock2 mode. This will also
    /// enable analog mode on DualShock1 controllers.
    pub fn enable_pressure(&mut self) -> Result<(), E> {
        // TODO: Redefine this to allow input parameters. Right now they're are hard coded
        // TODO: Detect and return actual protocol errors

        let mut buffer = [0u8; MESSAGE_MAX_LENGTH];

        // Wake up the controller if needed
        self.send_command(CMD_POLL, &mut buffer)?;

        self.send_command(CMD_ENTER_ESCAPE_MODE, &mut buffer)?;
        self.send_command(CMD_SET_MODE, &mut buffer)?;
        self.send_command(CMD_INIT_PRESSURE, &mut buffer)?;
        self.send_command(CMD_RESPONSE_FORMAT, &mut buffer)?;
        self.send_command(CMD_EXIT_ESCAPE_MODE, &mut buffer)?;

        Ok(())
    }

    /// Configure the JogCon for wheel control.
    /// 
    /// If no digital buttons are pressed in this mode for 60 seconds, the
    /// JogCon will go to sleep until buttons are pressed. If no polling is
    /// done for 10 seconds, it will drop out of this mode and revert to
    /// the standard Controller mode
    pub fn enable_jogcon(&mut self) -> Result<(), E> {
        let mut buffer = [0u8; MESSAGE_MAX_LENGTH];

        // Wake up the controller if needed
        self.send_command(CMD_POLL, &mut buffer)?;

        self.send_command(CMD_ENTER_ESCAPE_MODE, &mut buffer)?;
        self.send_command(CMD_SET_MODE, &mut buffer)?;
        self.send_command(CMD_MOTOR_JOGCON, &mut buffer)?;
        self.send_command(CMD_EXIT_ESCAPE_MODE, &mut buffer)?;

        Ok(())
    }

    /// Control the JogCon's jogwheel.
    /// 
    /// * `strength` - A value between 0 and 15. Any higher will wrap around.
    pub fn control_jogcon(&mut self, control: JogControl, strength: u8) -> Result<(), E> {
        let mut command = [0u8; MESSAGE_MAX_LENGTH];
        let mut buffer = [0u8; MESSAGE_MAX_LENGTH];

        let mut control: u8 = match control {
            JogControl::Hold => JogControl::Hold as u8,
            JogControl::Left => JogControl::Left as u8,
            JogControl::Right => JogControl::Right as u8,
            JogControl::Unknown1 => JogControl::Unknown1 as u8,
            JogControl::Unknown2 => JogControl::Unknown2 as u8,
            JogControl::Unknown3 => JogControl::Unknown3 as u8,
            _ => JogControl::Stop as u8
        };

        control |= strength & 0x0f;

        command[..CMD_POLL.len()].copy_from_slice(CMD_POLL);
        command[3] = control;

        self.send_command(&command, &mut buffer)?;

        Ok(())
    }

    /// Read various parameters from the controller including its current
    /// status.
    pub fn read_config(&mut self) -> Result<ControllerConfiguration, E> {
        let mut config: ControllerConfiguration = Default::default();
        let mut buffer = [0u8; MESSAGE_MAX_LENGTH];

        self.send_command(CMD_ENTER_ESCAPE_MODE, &mut buffer)?;

        self.send_command(CMD_READ_STATUS, &mut buffer)?;
        config.status.copy_from_slice(&mut buffer[HEADER_LEN..9]);

        self.send_command(CMD_READ_CONST1A, &mut buffer)?;
        config.const1a.copy_from_slice(&mut buffer[4..9]);

        self.send_command(CMD_READ_CONST1B, &mut buffer)?;
        config.const1b.copy_from_slice(&mut buffer[4..9]);

        self.send_command(CMD_READ_CONST2, &mut buffer)?;
        config.const2.copy_from_slice(&mut buffer[4..9]);

        self.send_command(CMD_READ_CONST3A, &mut buffer)?;
        config.const3a.copy_from_slice(&mut buffer[4..9]);

        self.send_command(CMD_READ_CONST3B, &mut buffer)?;
        config.const3b.copy_from_slice(&mut buffer[4..9]);

        self.send_command(CMD_EXIT_ESCAPE_MODE, &mut buffer)?;

        Ok(config)
    }

    /// Ask the controller for input states. Different contoller types can be returned.
    pub fn read_input(&mut self) -> Result<Device, Error<E>> {
        let mut buffer = [0u8; MESSAGE_MAX_LENGTH];
        let mut data = [0u8; MESSAGE_MAX_LENGTH];

        self.send_command(CMD_POLL, &mut buffer)?;
        data[0 .. MESSAGE_MAX_LENGTH - 3].copy_from_slice(&buffer[3..]);

        let controller = ControllerData { data: data };
        let device;

        unsafe {
            device = match buffer[1] {
                CONTROLLER_NOT_PRESENT => Device::None,
                CONTROLLER_CONFIGURATION => Device::ConfigurationMode,
                CONTROLLER_CLASSIC => Device::Classic(controller.classic),
                CONTROLLER_DUALSHOCK_DIGITAL => Device::Classic(controller.classic),                
                CONTROLLER_DUALSHOCK_ANALOG => Device::DualShock(controller.ds),
                CONTROLLER_DUALSHOCK_PRESSURE => Device::DualShock2(controller.ds2),
                CONTROLLER_JOGCON => Device::JogCon(controller.jc),
                CONTROLLER_NEGCON => Device::NegCon(controller.nc),
                _ => Device::Unknown,
            }
        }

        // Device polling will return `ACK_BYTE` in the third byte if the command
        // was properly understood
        /*
        match device {
            Device::None => {},
            _ => {
                if buffer[2] != ACK_BYTE {
                    return Err(Error::BadResponse);
                }
            }
        }
        */

        Ok(device)
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
