//! `NcPalette*`

// -----------------------------------------------------------------------------
// Now none of these functions can't fail and therefore don't return errors.
// -----------------------------------------------------------------------------
//
// functions already exported by bindgen : 3
// -----------------------------------------
// (#) test: 0
// (W) wrap: 3 / 0
// -----------------------------------------
//W  ncpalette_free
//W  ncpalette_new
//W  ncpalette_use
//
// functions manually reimplemented: 4
// -----------------------------------------
// (+) done: 3 / 0
// (#) test: 0
// (W) wrap: 3 / 0
// -----------------------------------------
//W+ ncpalette_get
//W+ ncpalette_get_rgb8
//W+ ncpalette_set
//W+ ncpalette_set_rgb8

use crate::c_api::ffi;

mod methods;
pub(crate) mod reimplemented;
pub use methods::*;

/// An array of 256 [`NcChannel`][crate::NcChannel]s.
///
/// See also [`NcPaletteIndex`].
///
/// Some terminals only support 256 colors, but allow the full
/// palette to be specified with arbitrary RGB colors. In all cases, it's more
/// performant to use indexed colors, since it's much less data to write to the
/// terminal. If you can limit yourself to 256 colors, that's probably best.
///
/// `type in C: ncncpalette (struct)`
pub type NcPalette = ffi::ncpalette;

/// Used for indexing into a [`NcPalette`] (alias of `u8`).
pub type NcPaletteIndex = u8;

impl NcPalette {
    /// The supported palette-indexed colors number is up to 8 bits.
    pub const SIZE: u32 = c_api::NCPALETTE_SIZE;
}

pub(crate) mod c_api {
    use super::ffi;

    /// The supported palette-indexed colors number is up to 8 bits.
    pub const NCPALETTE_SIZE: u32 = ffi::NCPALETTESIZE;
}
