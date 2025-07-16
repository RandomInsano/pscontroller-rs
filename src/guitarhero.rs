//! RedOctane's Guitar Hero Controller
//! ============================
//! Made for a single game, from a software point of view it's identical
//! to a DualShock1 controller. There's no way to tell it apart yet, so
//! the design of this library doesn't make it easy to use yet

/// Buttons on the Guitar Hero guitar. Reference material:
/// https://strategywiki.org/wiki/Guitar_Hero_II/Controls
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GuitarButtons {
    data: u16,
}

impl GuitarButtons {
    const PS_SELECT: u16 = 0x0001;
    const PS_START: u16 = 0x0008;

    const STRUM_UP: u16 = 0x0010;
    const STRUM_DOWN: u16 = 0x0040;

    const STAR_POWER: u16 = 0x0100;

    const FRET_GREEN: u16 = 0x0200;
    const FRET_RED: u16 = 0x2000;
    const FRET_YELLOW: u16 = 0x1000;
    const FRET_BLUE: u16 = 0x4000;
    const FRET_ORANGE: u16 = 0x8000;

    /// A button on the controller
    pub fn select(&self) -> bool {
        self.data & Self::PS_SELECT == 0
    }

    /// A button on the controller
    pub fn start(&self) -> bool {
        self.data & Self::PS_START == 0
    }

    /// A button on the controller
    pub fn strum_up(&self) -> bool {
        self.data & Self::STRUM_UP == 0
    }

    /// A button on the controller
    pub fn strum_down(&self) -> bool {
        self.data & Self::STRUM_DOWN == 0
    }

    /// A button on the controller
    pub fn fret_green(&self) -> bool {
        self.data & Self::FRET_GREEN == 0
    }

    /// A button on the controller
    pub fn fret_red(&self) -> bool {
        self.data & Self::FRET_RED == 0
    }

    /// A button on the controller
    pub fn fret_yellow(&self) -> bool {
        self.data & Self::FRET_YELLOW == 0
    }

    /// A button on the controller
    pub fn fret_blue(&self) -> bool {
        self.data & Self::FRET_BLUE == 0
    }

    /// A button on the controller
    pub fn fret_orange(&self) -> bool {
        self.data & Self::FRET_ORANGE == 0
    }

    /// A button on the controller
    pub fn star_power(&self) -> bool {
        self.data & Self::STAR_POWER == 0
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Represents a Guitar Hero controller
pub struct GuitarHero {
    /// The buttons
    pub buttons: GuitarButtons,

    // Lazily pad bytes
    padding: [u8; 3],

    /// The whammy bar's current position
    pub whammy: u8,
}

impl GuitarHero {
    /// Get a copy of the buttons that were pressed
    pub fn buttons(&self) -> GuitarButtons {
        self.buttons
    }
}
