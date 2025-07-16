//! Namco's GunGon Controller
//! ============================
//! A light gun for games like Time Crisis

use byteorder::{ByteOrder, LittleEndian};

/// The buttons found on the generation 1 GunCon. Once I find a GC2
/// I'll fill this out better
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GunconButtons {
    data: u16,
}

impl GunconButtons {
    const GC_A: u16 = 0x0008;
    const GC_B: u16 = 0x4000;
    const GC_TRIGGER: u16 = 0x2000;

    /// A button on the controller
    pub fn a(&self) -> bool {
        self.data & Self::GC_A == 0
    }

    /// A button on the controller
    pub fn b(&self) -> bool {
        self.data & Self::GC_B == 0
    }

    /// A button on the controller
    pub fn trigger(&self) -> bool {
        self.data & Self::GC_TRIGGER == 0
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Represents the classic Controller
pub struct GunCon {
    /// Standard buttons (Cross, Circle, L3, Start)
    pub buttons: GunconButtons,
    /// Location on screen (left-right)
    x: [u8; 2],
    /// Location on screen (up-down)
    y: [u8; 2],
}

impl GunCon {
    /// Return the X position of the GunCon's aim on the screen
    pub fn x(&self) -> u16 {
        LittleEndian::read_u16(&self.x)
    }

    /// Return the Y position of the GunCon's aim on the screen
    pub fn y(&self) -> u16 {
        LittleEndian::read_u16(&self.y)
    }
}
