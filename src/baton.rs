//! G.A E., Inc. GEA001 Controller
//! ============================
//! A musical baton controller with accelerometers for playing musical conductor
//! games such as Mad Maestro

/// The two buttons found on the baton
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BatonButtons {
    data: u16,
}

impl BatonButtons {
    const B_A: u16 = 0x0008;
    const B_B: u16 = 0x2000;

    /// A button on the controller
    pub fn a(&self) -> bool {
        self.data & Self::B_A == 0
    }

    /// A button on the controller
    pub fn b(&self) -> bool {
        self.data & Self::B_B == 0
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Represents the classic Controller
pub struct Baton {
    /// Standard buttons
    pub buttons: BatonButtons,
    /// Acceleration towards / away from your body
    pub z: u8,
    /// Unknown, assuming acceleration
    pub x: u8,
    /// Unknown, assuming acceleration
    pub y: u8,
    /// Unknown, assuming acceleration
    pub a: u8,
}
