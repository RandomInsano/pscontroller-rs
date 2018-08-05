//! Namco's GunGon Controller
//! ============================
//! A light gun for games like Time Crisis

use byteorder::{ByteOrder, BigEndian, LittleEndian};

#[repr(C)]
/// Represents the classic Controller
pub struct GunCon {
    // TODO: Check endianness of X and Y

    /// Standard buttons (Cross, Circle, L3, Start)
    pub buttons: u16,
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