//! Sony's DualShock Controllers
//! ============================
//! These guys everyone is familiar with, so for now there isn't
//! much of a description here!

use super::{
    HasStandardButtons,
    GamepadButtons
};

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
