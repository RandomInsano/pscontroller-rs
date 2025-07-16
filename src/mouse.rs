//! PlayStation Mouse
//! ============================
//! That [standard looking mouse](https://en.wikipedia.org/wiki/PlayStation_Mouse)
//! for the PlayStation. This was implemented from notes online and while it
//! should be accurate, it has not been tested.

/// The two buttons found on the mouse
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MouseButtons {
    data: u16,
}

impl MouseButtons {
    const PM_L: u16 = 0x0800;
    const PM_R: u16 = 0x0400;

    /// A button on the controller
    pub fn left(&self) -> bool {
        self.data & Self::PM_L == 0
    }

    /// A button on the controller
    pub fn right(&self) -> bool {
        self.data & Self::PM_R == 0
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Represents the classic Controller
pub struct Mouse {
    /// Standard buttons
    pub buttons: MouseButtons,
    /// Difference in Y-Axis since last poll
    pub y: i8,
    /// Difference in X-Axis since last poll
    pub x: i8,
}
