//! RedOctane's Guitar Hero Controller
//! ============================
//! Made for a single game, from a software point of view it's identical
//! to a DualShock1 controller. There's no way to tell it apart yet, so
//! the design of this library doesn't make it easy to use yet

#[repr(C)]
/// Represents a Guitar Hero controller
pub struct GuitarHero {
    // TODO: Figure out GH's button layout (strum up/down, fret colours)

    /// The buttons, currently not mapped
    pub buttons: u16,

    // Lazily pad bytes
    padding: [u8; 3],

    /// The whammy bar's current position
    pub whammy: u8,
}
