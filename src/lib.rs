//! Playstation Controller driver for Rust's [Embedded Hardware Abstraction Layer](https://github.com/japaric/embedded-hal)
//! ============================
//!
//! The original PlayStation and most of its peripherals are 20+ years old at this point,
//! but they're easy to interface with for fun projects and wireless variants are easy
//! to come by while being [pretty cheap](https://www.amazon.ca/s/?ie=UTF8&keywords=ps2+wireless+controller)
//! ($28 Canadian for two as of this writing).
//!
//! The current state of this library is such that it's pretty naïve to which controller
//! is plugged in, and it will make a guess based on the response to a poll request. The
//! response is broken into a header with an identification byte and an acknowledge byte and
//! we use the whole identification byte and just assume the data its sending is correct.
//!
//! If you own something particularly interesting for the PS1 or PS2 that plugs into the
//! controller port and isn't supported here, feel free to reach out by creating an issue
//! and we can work out some creative way to get the device working with this library.
//!
//! Efficiencies can be made here, and things will likely improve, but the darn thing is
//! useful now so let's start using it! If you find things to fix, please make an issue.
//!
//! Hardware
//! -----------------------
//!
//! Because the PlayStation can have up to four devices sharing the same SPI bus (two
//! controllers and two memory cards), they made the data out (MISO) pin open drain. That
//! means that it can only pull the line low and you'll need to add your own resistor
//! connected from that pin to +5v. In my testing with a voltage divider on a PlayStation 2,
//! the value is between 220 and 500 ohms.
//!
//! Development and testing is recommended to by done on a Raspberry Pi as it has a reliable
//! SPI bus. Early testing on both a Next Thing Co. C.H.I.P. and an Odroid C1+ didn't go
//! very well. The C.H.I.P. added unexpected clock changes, and the C1+ had voltage pull
//! up/down problems with the data lines.
//!
//! The controller itself is a 3.3v logic device and any force feedback is meant to be
//! driven by the 7.5v cd-rom voltage. Testing using the 5v line on of the Raspberry Pi
//! technically works, but the result is much weaker than the original console.
//!
//! Bibliography
//! -----------------------
//! Here is the list of the great bits of documentation that helped get this project started
//! and continue to provide a good cross-reference to double check the work done here.
//!
//! * [psxpad.html](http://domisan.sakura.ne.jp/article/psxpad/psxpad.html) - Wiring, testing and bootstrapping game pad on Linux with SPI
//! * [ps_eng.txt](http://kaele.com/~kashima/games/ps_eng.txt) - Controller / Memory Card Protocols (pre-DualShock)
//! * [Playstation 2 (Dual Shock) controller protocol notes](https://gist.github.com/scanlime/5042071) - Command protocols
//! * [psxpblib](http://www.debaser.force9.co.uk/psxpblib/) - Interfacing PlayStation controllers via the parallel port
//! * [Simulated PS2 Controller for Autonomously playing Guitar Hero](http://procrastineering.blogspot.ca/2010/12/simulated-ps2-controller-for.html) - SPI protocol captures

#![no_std]
#![deny(missing_docs)]

pub mod baton;
pub mod classic;
pub mod dualshock;
pub mod guitarhero;
pub mod guncon;
pub mod jogcon;
pub mod mouse;
pub mod negcon;

extern crate bit_reverse;
extern crate bitflags;
extern crate byteorder;
extern crate embedded_hal as hal;

use bit_reverse::ParallelReverse;
use core::fmt;
use hal::digital::OutputPin;
use hal::spi::SpiBus;

use baton::Baton;
use classic::{Classic, GamepadButtons};
use dualshock::{DualShock, DualShock2};
use guitarhero::GuitarHero;
use guncon::GunCon;
use jogcon::JogCon;
use mouse::Mouse;
use negcon::NegCon;

/// The maximum length of a message from a controller
const MESSAGE_MAX_LENGTH: usize = 32;
/// Acknoweldgement byte for header commnad
//const ACK_BYTE: u8 = 0x5a;
/// Length of the command header
const HEADER_LEN: usize = 3;

/// Controller missing
const CONTROLLER_NOT_PRESENT: u8 = 0xff;
/// PlayStation mouse, SCPH-1030
const CONTROLLER_MOUSE: u8 = 0x12;
/// Original controller, SCPH-1080
const CONTROLLER_CLASSIC: u8 = 0xc1;
/// Analog Controller, SCPH-1110 (flightstick looking thing)
const CONTROLLER_ANALOG_JOYSTICK: u8 = 0x53;
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
/// NegCon
const CONTROLLER_GUNCON: u8 = 0x63;
/// Configuration Mode
const CONTROLLER_CONFIGURATION: u8 = 0xf3;

/// Command to poll buttons
const CMD_POLL: &[u8] = &[0x00, 0x42, 0x00];
/// Command to enter escape mode
const CMD_ENTER_ESCAPE_MODE: &[u8] = &[0x00, 0x43, 0x00, 0x01, 0x00];
/// Command to exit escape mode
const CMD_EXIT_ESCAPE_MODE: &[u8] = &[0x00, 0x43, 0x00, 0x00, 0x00];
/// Command to set response format. Right now asks for all data
const CMD_RESPONSE_FORMAT: &[u8] = &[0x00, 0x4F, 0x00, 0xFF, 0xFF, 0x03, 0x00, 0x00, 0x00];
/// Command to initialize / customize pressure
const CMD_INIT_PRESSURE: &[u8] = &[0x00, 0x40, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00];
/// Command to set major mode (DualShock = 1 / Digital = 0)
const CMD_SET_MODE: &[u8] = &[0x00, 0x44, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00];
/// Command to read extended status
const CMD_READ_STATUS: &[u8] = &[0x00, 0x45, 0x00, 0x5a, 0x5a, 0x5a, 0x5a, 0x5a, 0x5a];
/// Command to read constant 1 at address 00
const CMD_READ_CONST1A: &[u8] = &[0x00, 0x46, 0x00, 0x00, 0x5a, 0x5a, 0x5a, 0x5a, 0x5a];
/// Command to read constant 1 at address 01
const CMD_READ_CONST1B: &[u8] = &[0x00, 0x46, 0x00, 0x01, 0x5a, 0x5a, 0x5a, 0x5a, 0x5a];
/// Command to read constant 2 at address 00
const CMD_READ_CONST2: &[u8] = &[0x00, 0x47, 0x00, 0x00, 0x5a, 0x5a, 0x5a, 0x5a, 0x5a];
/// Command to read constant 3 at address 00
const CMD_READ_CONST3A: &[u8] = &[0x00, 0x4C, 0x00, 0x00, 0x5a, 0x5a, 0x5a, 0x5a, 0x5a];
/// Command to read constant 3 at address 01
const CMD_READ_CONST3B: &[u8] = &[0x00, 0x4C, 0x00, 0x01, 0x5a, 0x5a, 0x5a, 0x5a, 0x5a];
/// Command to enable DualShock motors
const CMD_MOTOR_DUALSHOCK: &[u8] = &[0x00, 0x4D, 0x00, 0x00, 0x01, 0xff, 0xff, 0xff, 0xff];
/// Command to enable JogCon motor
const CMD_MOTOR_JOGCON: &[u8] = &[0x00, 0x4D, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff];

#[repr(C)]
#[derive(Copy, Clone)]
/// The poll command returns a series of bytes. This union allows us to interact with
/// those bytes with some functions. It's nice sugar that allows us to use the data
/// in an obvious way without needing to copy the bytes around
pub union ControllerData {
    /// The raw data representing the buttons. Kind of unwieldy
    pub data: [u8; MESSAGE_MAX_LENGTH],
    /// Maps the underlying bytes to buttons and the whammy bar of the Guitar Hero controller
    pub gh: GuitarHero,
    /// Maps the bytes to the Mad Maestro baton
    pub b: Baton,
    pm: Mouse,
    classic: Classic,
    ds: DualShock,
    ds2: DualShock2,
    jc: JogCon,
    nc: NegCon,
    gc: GunCon,
}

/// The active port to set on the Multitap
#[derive(Clone, PartialEq)]
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
    /// Some unknown id when booting the PS2. Memory card?
    M = 0x61,
    /// This may be for the multitap itself
    X = 0xff,
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

impl<E> fmt::Debug for Error<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error")
    }
}

/// Commands to send off with a poll request.
pub trait PollCommand {
    /// Re-write the provided slice starting from index 0. This command
    /// is called by read_input() which will provide a sub-slice of the
    /// controller's command bytes.
    fn set_command(&self, _: &mut [u8]);
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
    /// PlayStation mouse released at launch time with the original PlayStation
    Mouse(Mouse),
    /// Original controller that shipped with the PlayStation. Only contains regular
    /// buttons. DualShock 1 and 2 can emulate this mode
    Classic(Classic),
    /// Sony's strange flight-stick looking thing. Maps to the same data as the
    /// DualShock 1 but has a different identifier (fun fact: it predates the
    /// DualShock)
    AnalogJoystick(DualShock),
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
    /// The Namco GunCon
    GunCon(GunCon),
    /// The Mad Maestro Baton
    Baton(Baton),
}

/// The main event! Create a port using an SPI bus and start commanding
/// controllers!
pub struct PlayStationPort<SPI, CS> {
    dev: SPI,
    select: Option<CS>,
    multitap_port: MultitapPort,
}

impl<SPI, CS> PlayStationPort<SPI, CS>
where
    SPI: SpiBus,
    CS: OutputPin,
{
    /// Create a new device to talk over the PlayStation's controller
    /// port
    pub fn new(spi: SPI, mut select: Option<CS>) -> Self {
        // If a select pin was provided, disable the controller for now
        if let Some(ref mut x) = select {
            let _ = x.set_high();
        }

        Self {
            dev: spi,
            select,
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
    pub fn send_command(&mut self, command: &[u8], result: &mut [u8]) -> Result<(), SPI::Error> {
        // Pack in bytes for the command we'll be sending
        result[..command.len()].copy_from_slice(command);
        result[0] = self.multitap_port.clone() as u8;

        // Because not all hardware supports LSB mode for SPI, we flip
        // the bits ourselves
        Self::flip(result);

        if let Some(ref mut x) = self.select {
            let _ = x.set_low();
        }

        self.dev.transfer_in_place(result)?;

        if let Some(ref mut x) = self.select {
            let _ = x.set_high();
        }

        Self::flip(result);

        Ok(())
    }

    /// Configure the controller to set it to DualShock2 mode. This will also
    /// enable analog mode on DualShock1 controllers.
    pub fn enable_pressure(&mut self) -> Result<(), SPI::Error> {
        // TODO: Redefine this to allow input parameters. Right now they're are hard coded
        // TODO: Detect and return actual protocol errors

        let mut buffer = [0u8; MESSAGE_MAX_LENGTH];

        // Wake up the controller if needed
        self.send_command(CMD_POLL, &mut buffer)?;

        self.send_command(CMD_ENTER_ESCAPE_MODE, &mut buffer)?;
        self.send_command(CMD_SET_MODE, &mut buffer)?;
        self.send_command(CMD_MOTOR_DUALSHOCK, &mut buffer)?;
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
    pub fn enable_jogcon(&mut self) -> Result<(), SPI::Error> {
        let mut buffer = [0u8; MESSAGE_MAX_LENGTH];

        // Wake up the controller if needed
        self.send_command(CMD_POLL, &mut buffer)?;

        self.send_command(CMD_ENTER_ESCAPE_MODE, &mut buffer)?;
        self.send_command(CMD_SET_MODE, &mut buffer)?;
        self.send_command(CMD_MOTOR_JOGCON, &mut buffer)?;
        self.send_command(CMD_EXIT_ESCAPE_MODE, &mut buffer)?;

        Ok(())
    }

    /// Read various parameters from the controller including its current
    /// status.
    pub fn read_config(&mut self) -> Result<ControllerConfiguration, SPI::Error> {
        let mut config: ControllerConfiguration = Default::default();
        let mut buffer = [0u8; MESSAGE_MAX_LENGTH];

        self.send_command(CMD_ENTER_ESCAPE_MODE, &mut buffer)?;

        self.send_command(CMD_READ_STATUS, &mut buffer)?;
        config.status.copy_from_slice(&buffer[HEADER_LEN..9]);

        self.send_command(CMD_READ_CONST1A, &mut buffer)?;
        config.const1a.copy_from_slice(&buffer[4..9]);

        self.send_command(CMD_READ_CONST1B, &mut buffer)?;
        config.const1b.copy_from_slice(&buffer[4..9]);

        self.send_command(CMD_READ_CONST2, &mut buffer)?;
        config.const2.copy_from_slice(&buffer[4..9]);

        self.send_command(CMD_READ_CONST3A, &mut buffer)?;
        config.const3a.copy_from_slice(&buffer[4..9]);

        self.send_command(CMD_READ_CONST3B, &mut buffer)?;
        config.const3b.copy_from_slice(&buffer[4..9]);

        self.send_command(CMD_EXIT_ESCAPE_MODE, &mut buffer)?;

        Ok(config)
    }

    fn read_port(
        &mut self,
        command: Option<&dyn PollCommand>,
    ) -> Result<[u8; MESSAGE_MAX_LENGTH], Error<SPI::Error>> {
        let mut buffer = [0u8; MESSAGE_MAX_LENGTH];
        let mut data = [0u8; MESSAGE_MAX_LENGTH];

        data[..CMD_POLL.len()].copy_from_slice(CMD_POLL);

        // Overlay the command to send with the poll...
        if let Some(x) = command {
            x.set_command(&mut data[HEADER_LEN..]);
        }

        self.send_command(&data, &mut buffer)?;

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

        Ok(buffer)
    }

    /// Get the raw data from polling for a controller. You can use this to cooerce the data into
    /// some controller that can't be safely identified by `read_input`, but you should rely on that
    /// function if you can.
    pub fn read_raw(
        &mut self,
        command: Option<&dyn PollCommand>,
    ) -> Result<ControllerData, Error<SPI::Error>> {
        let mut buffer = [0u8; MESSAGE_MAX_LENGTH];
        let data = self.read_port(command)?;

        // Shift the controller data over because we don't need the header anymore
        buffer[0..MESSAGE_MAX_LENGTH - 3].copy_from_slice(&data[HEADER_LEN..]);

        // FIXME: This doesn't zero out the end so we could theoretically have
        //        repeated data

        Ok(ControllerData { data: buffer })
    }

    /// Ask the controller for input states. Different contoller types will be returned automatically
    /// for you. If you'd like to cooerce a controller yourself, use `read_raw`.
    pub fn read_input(
        &mut self,
        command: Option<&dyn PollCommand>,
    ) -> Result<Device, Error<SPI::Error>> {
        let mut buffer = [0u8; MESSAGE_MAX_LENGTH];
        let data = self.read_port(command)?;

        // Shift the controller data over because we don't need the header anymore
        buffer[0..MESSAGE_MAX_LENGTH - 3].copy_from_slice(&data[HEADER_LEN..]);

        let controller = ControllerData { data: buffer };
        let device;

        unsafe {
            device = match data[1] {
                CONTROLLER_NOT_PRESENT => Device::None,
                CONTROLLER_CONFIGURATION => Device::ConfigurationMode,
                CONTROLLER_MOUSE => Device::Mouse(controller.pm),
                CONTROLLER_CLASSIC => Device::Classic(controller.classic),
                CONTROLLER_ANALOG_JOYSTICK => Device::AnalogJoystick(controller.ds),
                CONTROLLER_DUALSHOCK_DIGITAL => Device::Classic(controller.classic),
                CONTROLLER_DUALSHOCK_ANALOG => Device::DualShock(controller.ds),
                CONTROLLER_DUALSHOCK_PRESSURE => Device::DualShock2(controller.ds2),
                CONTROLLER_JOGCON => Device::JogCon(controller.jc),
                CONTROLLER_NEGCON => Device::NegCon(controller.nc),
                CONTROLLER_GUNCON => Device::GunCon(controller.gc),
                _ => Device::Unknown,
            }
        }

        Ok(device)
    }
}

#[cfg(test)]
mod tests {
    use super::ControllerData;
    use super::MESSAGE_MAX_LENGTH;

    #[test]
    fn union_test() {
        // Again, buttons are active low, hence 'fe' and '7f'
        let mut data = [0u8; MESSAGE_MAX_LENGTH];
        data[0] = 0xfe;
        data[1] = 0x7f;
        data[2] = 0x00;
        data[3] = 0x00;
        data[4] = 0x00;
        data[5] = 0xff;
        let controller = ControllerData { data };

        unsafe {
            assert!(controller.ds.buttons.select() == true);
            assert!(controller.ds.buttons.square() == true);
            assert!(controller.ds.lx == 0);
            assert!(controller.ds.ly == 255);
        }
    }
}
