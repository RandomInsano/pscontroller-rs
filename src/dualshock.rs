//! Sony's DualShock Controllers
//! ============================
//! These guys everyone is familiar with, so for now there isn't
//! much of a description here!
//! 
//! This also maps for the the Dual Analog (precursor to the Dual Shock) and
//! the Analog controller (flight stick) as they both have the same buttons

use crate::classic::GamepadButtons;
use super::{
    HasStandardButtons,
    PollCommand
};

#[repr(C)]
#[derive(Copy, Clone)]
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
        self.buttons
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
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
        self.buttons
    }
}

/// Command for controlling the vibration motors in the
/// dual shock controllers
pub struct ControlDS {
    /// Whether to turn on the small motor
    pub little: bool,
    /// How strong to run the large motor
    pub big: u8,
}

impl ControlDS {
    /// Create a new one of thes newfangled control commands
    pub fn new(little: bool, big: u8) -> Self {
        Self {
            little,
            big,
        }
    }
}

/// Implement the needed functions to control the motors on
/// the DualShock controllers
impl PollCommand for ControlDS {
    /// Sets the command for the rumble motoros on the DualShock
    fn set_command(&self, command: &mut [u8]) {
        command[0] = if self.little { 0xff } else { 0x00 };
        command[1] = self.big;
    }
}