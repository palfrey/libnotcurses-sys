//! `NcChannel*`

// -----------------------------------------------------------------------------
// - The channel components are u8 instead of u32.
//   Because of type enforcing, some runtime checks are now unnecessary.
//
// - None of the functions can't fail now. The original checks for dirty bits
//   have been substitued by mask cleaning (bitwise and)
//
// - These functions were deemed unnecessary to implement:
//   - `channel_set_rgb_clipped()`
//   - `channels_set_fg_rgb8_clipped()`
//   - `channels_set_bg_rgb8_clipped()`
// -----------------------------------------------------------------------------
//
// functions manually reimplemented: 44
// ------------------------------------------
// (X) wont:  3
// (+) done: 36 / 0
// (#) test: 21
// (W) wrap: 41
// ------------------------------------------
//W# channel_alpha
//W# channel_b
//W# channel_default_p
//W# channel_g
//W# channel_palindex_p
//W# channel_r
//W# channel_rgb8
//W# channel_set
//W# channel_set_alpha
//W# channel_set_default
//W# channel_set_not_default         // not in the original C API
//W# channel_set_rgb8
// X channel_set_rgb_clipped         // not needed
//W# channels_bchannel
//W+ channels_bg_alpha
//W+ channels_bg_default_p
//W# channels_bg_palindex_p
//W+ channels_bg_rgb
//W+ channels_bg_rgb8
//W# channels_combine
//W# channels_fchannel
//W+ channels_fg_alpha
//W+ channels_fg_default_p
//W# channels_fg_palindex_p
//W+ channels_fg_rgb
//W+ channels_fg_rgb8
//W# channels_set_bchannel
//W+ channels_set_bg_alpha
//W+ channels_set_bg_default
//W  channels_set_bg_not_default     // not in the original C API
//W# channels_set_bg_palindex
//W+ channels_set_bg_rgb
//W+ channels_set_bg_rgb8
// X channels_set_bg_rgb8_clipped    // not needed
//W  channels_set_default            // not in the original C API
//W# channels_set_fchannel
//W+ channels_set_fg_alpha
//W+ channels_set_fg_default
//W  channels_set_fg_not_default     // not in the original C API
//W# channels_set_fg_palindex
//W+ channels_set_fg_rgb
//W+ channels_set_fg_rgb8
// X channels_set_fg_rgb8_clipped    // not needed
//W  channels_set_not_default        // not in the original C API

#[cfg(test)]
mod test;

#[allow(clippy::module_inception)]
mod channel;
mod channels;
pub(crate) mod reimplemented;
pub use {channel::NcChannel, channels::NcChannels};

pub(crate) mod c_api {
    use crate::c_api::ffi;

    #[allow(unused_imports)]
    use crate::{NcAlpha, NcChannel, NcChannels};

    /// 32 bits of context-dependent info containing [`NcRgb_u32`]
    /// + [`NcAlpha_u32`] + extra
    ///
    /// It's recommended to use [`NcChannel`][crate::NcChannel] instead.
    ///
    /// It is composed of:
    /// - a 24-bit [`NcRgb_u32`] value
    /// - plus 8 bits divided in:
    ///   - 2 bits of [`NcAlpha_u32`]
    ///   - 6 bits of context-dependent info
    ///
    /// The context details are documented in [`NcChannels_u64`].
    ///
    /// ## Diagram
    ///
    /// ```txt
    /// ~~AA~~~~ RRRRRRRR GGGGGGGG BBBBBBBB
    /// ```
    /// `type in C: channel (uint32_t)`
    ///
    /// # See also
    /// - [`NcRgb_u32`]
    /// - [`NcRgba_u32`]
    ///
    /// [`NcRgb_u32`]: crate::c_api::NcRgb_u32
    /// [`NcRgba_u32`]: crate::c_api::NcRgba_u32
    /// [`NcAlpha_u32`]: crate::c_api::NcAlpha_u32
    pub type NcChannel_u32 = u32;

    /// 64 bits containing a foreground and background [`NcChannel_u32`].
    ///
    /// It's recommended to use [`NcChannels`][crate::NcChannels] instead.
    ///
    /// At render time, both 24-bit [`NcRgb_u32`] values are quantized down to
    /// terminal capabilities, if necessary. There's a clear path to 10-bit
    /// support should we one day need it.
    ///
    /// ## Default Color
    ///
    /// The "default color" is best explained by [color(3NCURSES)][0] and
    /// [default_colors(3NCURSES)][1]. Ours is the same concept.
    ///
    /// [0]: https://manpages.debian.org/stretch/ncurses-doc/color.3ncurses.en.html
    /// [1]: https://manpages.debian.org/stretch/ncurses-doc/default_colors.3ncurses.en.html
    ///
    /// **Until the "not default color" bit is set, any color you load will be ignored.**
    ///
    /// ## Diagram
    ///
    /// ```txt
    /// ~~AA~~~~|RRRRRRRR|GGGGGGGG|BBBBBBBB║~~AA~~~~|RRRRRRRR|GGGGGGGG|BBBBBBBB
    /// ↑↑↑↑↑↑↑↑↑↑↑↑ foreground ↑↑↑↑↑↑↑↑↑↑↑║↑↑↑↑↑↑↑↑↑↑↑↑ background ↑↑↑↑↑↑↑↑↑↑↑
    /// ```
    ///
    /// Detailed info (specially on the context-dependent bits on each
    /// [`NcChannel_u32`]'s 4th byte):
    ///
    /// ```txt
    ///                             ~foreground channel~
    /// reserved, must be 0                                  ↓bits view↓               ↓hex mask↓
    /// 0·······|········|········|········║········|········|········|········  =  8·······|········
    ///
    /// NcChannels::FG_DEFAULT_MASK: foreground is NOT "default color"
    /// ·1······|········|········|········║········|········|········|········  =  4·······|········
    ///
    /// NcChannels::FG_ALPHA_MASK: foreground alpha (2bits)
    /// ··11····|········|········|········║········|········|········|········  =  3·······|········
    ///
    /// NcChannels::FG_PALETTE: foreground uses palette index
    /// ····1···|········|········|········║········|········|········|········  =  ·8······|········
    ///
    /// NcChannels::NOBACKGROUND_MASK: glyph is entirely foreground
    /// ·····1··|········|········|········║········|········|········|········  =  ·4······|········
    ///
    /// reserved, must be 0
    /// ······00|········|········|········║········|········|········|········  =  ·3······|········
    ///
    /// NcChannels::FG_RGB_MASK: foreground in 3x8 RGB (rrggbb)
    /// ········|11111111|11111111|11111111║········|········|········|········  =  ··FFFFFF|········
    /// ```
    ///
    /// ```txt
    ///                             ~background channel~
    /// reserved, must be 0                                  ↓bits view↓               ↓hex mask↓
    /// ········|········|········|········║0·······|········|········|········  =  ········|8·······
    ///
    /// NcChannels::BGDEFAULT_MASK: background is NOT "default color"
    /// ········|········|········|········║·1······|········|········|········  =  ········|4·······
    ///
    /// NcChannels::BG_ALPHA_MASK: background alpha (2 bits)
    /// ········|········|········|········║··11····|········|········|········  =  ········|3·······
    ///
    /// NcChannels::BG_PALETTE: background uses palette index
    /// ········|········|········|········║····1···|········|········|········  =  ········|·8······
    ///
    /// reserved, must be 0
    /// ········|········|········|········║·····000|········|········|········  =  ········|·7······
    ///
    /// NcChannels::BG_RGB_MASK: background in 3x8 RGB (rrggbb)
    /// ········|········|········|········║········|11111111|11111111|11111111  =  ········|··FFFFFF
    /// ```
    /// `type in C: channels (uint64_t)`
    ///
    /// ## `NcChannels` Mask Flags
    ///
    /// - [`NcChannels::BG_DEFAULT_MASK`][NcChannels#associatedconstant.BGDEFAULT_MASK]
    /// - [`NcChannels::BG_ALPHA_MASK`][NcChannels#associatedconstant.BG_ALPHA_MASK]
    /// - [`NcChannels::BG_PALETTE`][NcChannels#associatedconstant.BG_PALETTE]
    /// - [`NcChannels::BG_RGB_MASK`][NcChannels#associatedconstant.BG_RGB_MASK]
    /// - [`NcChannels::FG_DEFAULT_MASK`][NcChannels#associatedconstant.FGDEFAULT_MASK]
    /// - [`NcChannels::FG_ALPHA_MASK`][NcChannels#associatedconstant.FG_ALPHA_MASK]
    /// - [`NcChannels::FG_PALETTE`][NcChannels#associatedconstant.FG_PALETTE]
    /// - [`NcChannels::FG_RGB_MASK`][NcChannels#associatedconstant.FG_RGB_MASK]
    ///
    /// [`NcRgb_u32`]: crate::c_api::NcRgb_u32
    /// [`NcChannel_u32`]: crate::c_api::NcChannel_u32
    ///
    pub type NcChannels_u64 = u64;

    /// If this bit is set, we are *not* using the default background color
    ///
    /// Note: This is equivalent to
    /// [`NcChannel::DEFAULT_MASK`][NcChannel#associatedconstant.DEFAULT_MASK]
    ///
    /// See the detailed diagram at [`NcChannels`]
    pub const NC_BGDEFAULT_MASK: u32 = ffi::NC_BGDEFAULT_MASK;

    /// Extract these bits to get the (background) [`NcAlpha`] mask.
    ///
    /// Note: This is equivalent to
    /// [`NcChannel::ALPHA_MASK`][NcChannel#associatedconstant.ALPHA_MASK]
    ///
    /// See the detailed diagram at [`NcChannels`]
    pub const NC_BG_ALPHA_MASK: u32 = ffi::NC_BG_ALPHA_MASK;

    /// If this bit *and* [`NC_BGDEFAULT_MASK`] are set, we're using a
    /// palette-indexed background color
    ///
    /// Note: This is equivalent to
    /// [`NcChannel::PALETTE_MASK`][NcChannel#associatedconstant.PALETTE_MASK]
    ///
    /// See the detailed diagram at [`NcChannels`]
    pub const NC_BG_PALETTE: u32 = ffi::NC_BG_PALETTE;

    /// Extract these bits to get the background [`NcRgb`][crate::NcRgb] value
    ///
    /// Note: This is equivalent to
    /// [`NcChannel::RGB_MASK`][NcChannel#associatedconstant.RGB_MASK]
    ///
    /// See the detailed diagram at [`NcChannels`]
    pub const NC_BG_RGB_MASK: u32 = ffi::NC_BG_RGB_MASK;

    /// Does this glyph completely obscure the background?
    ///
    /// If so, there's no need to emit a background when rasterizing,
    /// a small optimization. These are also used to track regions into which
    /// we must not cellblit.
    pub const NC_NOBACKGROUND_MASK: u64 = ffi::NC_NOBACKGROUND_MASK as u64;
}
