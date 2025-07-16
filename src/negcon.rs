//! Namco's NeGcon Controller
//! ============================
//! A controller with twist! This controller has multiple analog buttons and twists in
//! the center. It doesn't follow the normal PlayStation button scheme, so it has its
//! own button helper struct to work through the buttons
//!
//! Testing
//! ------------
//! Testing for this module was done on a JogCon controller running in NeGcon
//! compatibility mode. It will work this way if the 'mode' button is held when the
//! controller is powered on or plugged in.

/// The digital buttons of the Namco NegCon
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NegconButtons {
    data: u16,
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

#[repr(C)]
#[derive(Copy, Clone)]
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
    pub switchl: u8,
}
