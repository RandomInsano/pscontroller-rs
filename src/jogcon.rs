//! Namco's JogCon Controller
//! ============================
//! This controller has a jog wheel in the center which is controlled by a
//! motor to provide force feedback. Only three games support the controller
//! but it is featureful enough that it can be treated as a servo motor with
//! little effort.

use super::{HasStandardButtons, PollCommand};
use crate::classic::GamepadButtons;
use byteorder::{ByteOrder, LittleEndian};

/// What we want the JogCon's wheel to do after we
/// poll it
#[derive(Clone)]
pub enum JogControl {
    /// Stop the motor
    Stop = 0x00,
    /// Hold the wheel in position (and return it if it moves)
    Hold = 0x30,
    /// Start turning the wheel left
    Left = 0x20,
    /// Start turning the wheel right
    Right = 0x10,
    /// Drop how many revolutions were turned and keep track of
    /// only the distance to return to the original angle
    DropRevolutions = 0x80,
    /// Drop how many revolutions were turned and return back
    /// to the starting angle
    DropAndHold = 0xb0,
    /// Set a new hold position
    NewHold = 0xc0,
}

/// What state the JogCon's wheel was in last poll
pub enum JogState {
    /// The wheel was turned left
    TurnedLeft,
    /// The wheel was turned right
    TurnedRight,
    /// The wheel met its maximum recordable distance
    AtMaximum,
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Represents the Namco JogCon controller
pub struct JogCon {
    // TODO: Implement an endian-safe accessor for jog_position
    // TODO: Implement an enum accessor for jog_state
    /// Standard buttons (Cross, Circle, L3, Start, etc)
    pub buttons: GamepadButtons,

    /// The absolute position of the jog wheel
    jog_position: [u8; 2],

    /// What state is the jog wheel in
    pub jog_state: u8,
}

impl JogCon {
    /// The absolute position of the jog wheel
    pub fn jog_position(&self) -> i16 {
        LittleEndian::read_i16(&self.jog_position)
    }
}

impl HasStandardButtons for JogCon {
    fn buttons(&self) -> GamepadButtons {
        self.buttons
    }
}

/// Command for controlling the wheel on the JogCon
pub struct ControlJC {
    /// The mode the wheel should be in (move left, move right, etc)
    pub mode: JogControl,
    /// How strong the motor should be working
    pub strength: u8,
}

impl ControlJC {
    /// Create a new one of thes newfangled control commands
    pub fn new(mode: JogControl, strength: u8) -> Self {
        Self { mode, strength }
    }
}

/// Implement the needed functions to control the motor on the JogCon
impl PollCommand for ControlJC {
    /// Sets the command for the wheel on the JogCon
    fn set_command(&self, command: &mut [u8]) {
        command[0] = self.mode.clone() as u8;
        command[0] |= self.strength & 0x0f;
    }
}
