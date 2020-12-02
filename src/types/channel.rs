#[allow(unused_imports)] // for docblocks
use crate::NcChar;

// NcChannel
//
/// 32 bits of context-dependent info
/// containing RGB + 2 bits of alpha + extra
///
/// It is:
/// - a 24-bit [`NcRgb`] value
/// - plus 8 bits divided in:
///   - 2 bits of [`NcAlphaBits`]
///   - 6 bits of context-dependent info
///
/// The context details are documented in [`NcChannelPair`]
///
/// ## Diagram
///
/// ```txt
/// ~~AA~~~~ RRRRRRRR GGGGGGGG BBBBBBBB
/// ```
/// `type in C: channel (uint32_t)`
///
pub type NcChannel = u32;

/// Extract these bits to get a channel's alpha value
pub const NCCHANNEL_ALPHA_MASK: u32 = crate::bindings::bindgen::CHANNEL_ALPHA_MASK;

// NcAlphaBits
//
/// 2 bits of alpha (surrounded by context dependent bits).
/// It is part of an [`NcChannel`].
///
/// ## Diagram
///
/// ```txt
/// ~~AA~~~~ -------- -------- --------
/// ```
///
/// `type in C: no data type`
///
pub type NcAlphaBits = u32;

// NcChannelPair
//
/// 64 bits containing a foreground and background [`NcChannel`]
///
/// At render time, both 24-bit [`NcRgb`] values are quantized down to terminal
/// capabilities, if necessary. There's a clear path to 10-bit support should
/// we one day need it.
///
/// ## Default Color
///
/// The "default color" is best explained by
/// [color(3NCURSES)](https://manpages.debian.org/stretch/ncurses-doc/color.3ncurses.en.html).
/// Ours is the same concept.
///
/// **Until the "not default color" bit is set, any color you load will be ignored.**
///
/// ## Diagram
///
/// ```txt
/// ~~AA~~~~|RRRRRRRR|GGGGGGGG|BBBBBBBB|~~AA~~~~|RRRRRRRR|GGGGGGGG|BBBBBBBB
/// ↑↑↑↑↑↑↑↑↑↑↑↑ foreground ↑↑↑↑↑↑↑↑↑↑↑|↑↑↑↑↑↑↑↑↑↑↑↑ background ↑↑↑↑↑↑↑↑↑↑↑
/// ```
///
/// Detailed info (specially on the context-dependent bits on each
/// [`NcChannel`]'s 4th byte):
///
/// ```txt
///                             ~foreground channel~
/// NCCELL_WIDEASIAN_MASK: part of a wide glyph          ↓bits view↓               ↓hex mask↓
/// 1······· ········ ········ ········ ········ ········ ········ ········  =  8······· ········
///
/// NCCELL_FGDEFAULT_MASK: foreground is NOT "default color"
/// ·1······ ········ ········ ········ ········ ········ ········ ········  =  4······· ········
///
/// NCCELL_FG_ALPHA_MASK: foreground alpha (2bits)
/// ··11···· ········ ········ ········ ········ ········ ········ ········  =  3······· ········
///
/// NCCELL_FG_PALETTE: foreground uses palette index
/// ····1··· ········ ········ ········ ········ ········ ········ ········  =  ·8······ ········
///
/// NCCELL_NOBACKGROUND_MASK: glyph is entirely foreground
/// ·····1·· ········ ········ ········ ········ ········ ········ ········  =  ·4······ ········
///
/// reserved, must be 0
/// ······00 ········ ········ ········ ········ ········ ········ ········  =  ·3······ ········
///
/// NCCELL_FG_RGB_MASK: foreground in 3x8 RGB (rrggbb)
/// ········ 11111111 11111111 11111111 ········ ········ ········ ········  =  ··FFFFFF ········
/// ```

/// ```txt
///                             ~background channel~
/// reserved, must be 0                                  ↓bits view↓               ↓hex mask↓
/// ········ ········ ········ ········ 0······· ········ ········ ········  =  ········ 8·······
///
/// NCCELL_BGDEFAULT_MASK: background is NOT "default color"
/// ········ ········ ········ ········ ·1······ ········ ········ ········  =  ········ 4·······
///
/// NCCELL_BG_ALPHA_MASK: background alpha (2 bits)
/// ········ ········ ········ ········ ··11···· ········ ········ ········  =  ········ 3·······
///
/// NCCELL_BG_PALETTE: background uses palette index
/// ········ ········ ········ ········ ····1··· ········ ········ ········  =  ········ ·8······
///
/// reserved, must be 0
/// ········ ········ ········ ········ ·····000 ········ ········ ········  =  ········ ·7······
///
/// NCCELL_BG_RGB_MASK: background in 3x8 RGB (rrggbb)
/// 0········ ········ ········ ········ ········11111111 11111111 11111111  =  ········ ··FFFFFF
/// ```
/// `type in C: channels (uint64_t)`
///
/// ## `NcCell` Mask Flags
///
/// - [`NCCELL_BGDEFAULT_MASK`][crate::NCCELL_BGDEFAULT_MASK]
/// - [`NCCELL_BG_ALPHA_MASK`][crate::NCCELL_BG_ALPHA_MASK]
/// - [`NCCELL_BG_PALETTE`][crate::NCCELL_BG_PALETTE]
/// - [`NCCELL_BG_RGB_MASK`][crate::NCCELL_BG_RGB_MASK]
/// - [`NCCELL_FGDEFAULT_MASK`][crate::NCCELL_FGDEFAULT_MASK]
/// - [`NCCELL_FG_ALPHA_MASK`][crate::NCCELL_FG_ALPHA_MASK]
/// - [`NCCELL_FG_PALETTE`][crate::NCCELL_FG_PALETTE]
/// - [`NCCELL_FG_RGB_MASK`][crate::NCCELL_FG_RGB_MASK]
/// - [`NCCELL_NOBACKGROUND_MASK`][crate::NCCELL_NOBACKGROUND_MASK]
/// - [`NCCELL_WIDEASIAN_MASK`][crate::NCCELL_WIDEASIAN_MASK]
///
pub type NcChannelPair = u64;

// NcRgb
//
/// 24 bits broken into 3x 8bpp channels.
///
/// Unlike with [`NcChannel`], operations involving `NcRgb` ignores the last 4th byte
///
/// ## Diagram
///
/// ```txt
/// -------- RRRRRRRR GGGGGGGG BBBBBBBB
/// ```
///
/// `type in C: no data type`
///
pub type NcRgb = u32;

// NcColor
//
/// 8 bits representing a R/G/B color or alpha channel
///
/// ## Diagram
///
/// ```txt
/// CCCCCCCC (1 Byte)
/// ```
///
/// `type in C: no data type`
///
pub type NcColor = u8;

// NcPixel (RGBA)
/// 32 bits broken into RGB + 8-bit alpha
///
/// NcPixel has 8 bits of alpha,  more or less linear, contributing
/// directly to the usual alpha blending equation.
///
/// We map the 8 bits of alpha to 2 bits of alpha via a level function:
/// https://nick-black.com/dankwiki/index.php?title=Notcurses#Transparency.2FContrasting
///
/// ## Diagram
///
/// ```txt
/// AAAAAAAA GGGGGGGG BBBBBBBB RRRRRRRR
/// ```
/// `type in C: ncpixel (uint32_t)`
///
// NOTE: the order of the colors is different than in NcChannel.
pub type NcPixel = u32;

/// NcPalette structure consisting of an array of 256 [`NcChannel`]s.
///
/// See also [NcPaletteIndex].
///
/// Some terminals only support 256 colors, but allow the full
/// palette to be specified with arbitrary RGB colors. In all cases, it's more
/// performant to use indexed colors, since it's much less data to write to the
/// terminal. If you can limit yourself to 256 colors, that's probably best.
///
/// `type in C: ncpalette256 (struct)`
///
pub type NcPalette = crate::bindings::bindgen::palette256;

/// 8-bit value used for indexing into a [`NcPalette`]
///
pub type NcPaletteIndex = u8;

/// Context for a palette fade operation
pub type NcFadeCtx = crate::bindings::bindgen::ncfadectx;

/// the [`NcChar`] which form the various levels
/// of a given geometry.
///
/// If the geometry is wide, things are arranged with the rightmost side
/// increasing most quickly, i.e. it can be indexed as height arrays of
/// 1 + height glyphs.
/// i.e. The first five braille EGCs are all 0 on the left,
/// [0..4] on the right.
///
/// `type in C: blitset (struct)`
///
pub type NcBlitSet = crate::bindings::bindgen::blitset;
