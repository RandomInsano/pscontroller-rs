//! Sony's Original PlayStation Controller
//! ============================
//! This is the predecessor of the DualShock and is the controller
//! which originally shipped with the original PlayStation

use super::HasStandardButtons;

/// The digital buttons of the gamepad
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GamepadButtons {
    data: u16,
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

#[repr(C)]
#[derive(Copy, Clone)]
/// Represents the classic Controller
pub struct Classic {
    /// Standard buttons (Cross, Circle, L3, Start)
    pub buttons: GamepadButtons,
}

impl HasStandardButtons for Classic {
    fn buttons(&self) -> GamepadButtons {
        self.buttons
    }
}