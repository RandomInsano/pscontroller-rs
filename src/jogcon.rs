//! Namco's JogCon Controller
//! ============================
//! This controller has a jog wheel in the center which is controlled by a
//! motor to provide force feedback. Only three games support the controller
//! but it is featureful enough that it can be treated as a servo motor with
//! little effort.

use super::{
    HasStandardButtons,
    GamepadButtons
};

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
