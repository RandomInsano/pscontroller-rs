//! Sony's Original PlayStation Controller
//! ============================
//! This is the predecessor of the DualShock and is the controller
//! which originally shipped with the original PlayStation

use super::{
    HasStandardButtons,
    GamepadButtons
};

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