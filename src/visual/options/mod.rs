//!

use crate::{c_api::ffi, NcBlitter, NcPlane, NcRgba, NcScale};
use core::ptr::null_mut;

mod builder;
pub use builder::NcVisualOptionsBuilder;

/// Options struct for [`NcVisual`][crate::NcVisual].
///
/// It is recommended to construct it via [`NcVisualOptionsBuilder`]
/// by calling [`NcVisualOptions::builder()`].
///
/// # Usage
///
/// If a plane is not provided, one will be created, having the exact size
/// necessary to display the visual (this might be smaller or larger than
/// the rendering area). if [`ChildPlane`] is provided, this will be
/// interpreted as the parent.
///
/// A subregion of the visual can be rendered using `beg_y`, `beg_x`, `len_y`,
/// and `len_x`.
///
/// # Fields
///
/// * [`n`] - an optional mutable pointer to an [`NcPlane`].
///
/// * [`scaling`] - an [`NcScale`] indicating how the source will be
///   stretched/scaled relative to the `NcPlane`.
///
/// * [`y`] - if an `NcPlane` is provided in `n` then this specifies where the
///   `NcVisual` will be on that plane.
///
///   Otherwise it specifies where the created `NcPlane` will be placed relative
///   to the standard plane's origin.
///
///   If [`VerAligned`] is set, this will be interpreted as an [`NcAlign`] value.
///
/// * [`x`] - if an `NcPlane` is provided in `n` then this specifies where the
///   `NcVisual` will be on that plane.
///
///   Otherwise it specifies where the created `NcPlane` will be placed relative
///   to the standard plane's origin.
///
///   If [`HorAligned`] is set, this will be interpreted as an [`NcAlign`] value.
///
/// * [`begy`] - origin of rendered section in the *y* axis.
/// * [`begx`] - origin of rendered section in the *x* axis.
/// * [`leny`] - length of rendered section in the *y* axis.
/// * [`lenx`] - length of rendered section in the *x* axis.
///
/// * [`blitter`] - [`NcBlitter`] glyph set to use for blitting.
///
/// * [`flags`] - [`NcVisualFlags`].
///
/// * [`transcolor`] - treats this color as transparent when the [`AddAlpha`]
///   flag is active.
///
/// * [`pxoffy`] - pixel offset within the cell in the *y* axis.
///
///   If [`NcBlitter::Pixel`] is used the bitmap will be drawn offset from the
///   upper-left cell’s origin by these amounts, otherwise this will be ignored.
///
///   It is an error if either number exceeds the cell-pixel geometry in any
///   dimension (see [`NcPixelGeometry.cell_y`], [`NcVisualGeometry.cdim_yx`]).
///
/// * [`pxoffx`] - pixel offset within the cell in the *x* axis.
///
///   If [`NcBlitter::Pixel`] is used, the bitmap will be drawn offset from the
///   upper-left cell’s origin by these amounts, otherwise this will be ignored.
///
///   It is an error if either number exceeds the cell-pixel geometry in any
///   dimension (see [`NcPixelGeometry.cell_x`], [`NcVisualGeometry.cdim_yx`]).
///
/// [`NcVisualOptions::builder()`]: NcVisualOptions#method.builder
/// [`NcAlign`]: crate::NcAlign
/// [`NcPixelGeometry.cell_y`]: crate::NcPixelGeometry#structfield.cell_y
/// [`NcPixelGeometry.cell_x`]: crate::NcPixelGeometry#structfield.cell_x
/// [`NcVisualGeometry.cdim_yx`]: crate::NcVisualGeometry#structfield.cdim_yx
/// [`n`]: crate::c_api::ffi::ncvisual_options#structfield.n
/// [`scaling`]: crate::c_api::ffi::ncvisual_options#structfield.scaling
/// [`y`]: crate::c_api::ffi::ncvisual_options#structfield.y
/// [`x`]: crate::c_api::ffi::ncvisual_options#structfield.x
/// [`begy`]: crate::c_api::ffi::ncvisual_options#structfield.begy
/// [`begx`]: crate::c_api::ffi::ncvisual_options#structfield.begx
/// [`leny`]: crate::c_api::ffi::ncvisual_options#structfield.leny
/// [`lenx`]: crate::c_api::ffi::ncvisual_options#structfield.lenx
/// [`blitter`]: crate::c_api::ffi::ncvisual_options#structfield.blitter
/// [`flags`]: crate::c_api::ffi::ncvisual_options#structfield.flags
/// [`transcolor`]: crate::c_api::ffi::ncvisual_options#structfield.transcolor
/// [`pxoffy`]: crate::c_api::ffi::ncvisual_options#structfield.pxoffy
/// [`pxoffx`]: crate::c_api::ffi::ncvisual_options#structfield.pxoffx
/// [`AddAlpha`]: NcVisualFlags#associatedconstant.AddAlpha
/// [`Childplane`]: NcVisualFlags#associatedconstant.Childplane
/// [`VerAligned`]:NcVisualFlags#associatedconstant.VerAligned
/// [`HorAligned`]: NcVisualFlags#associatedconstant.HorAligned
pub type NcVisualOptions = crate::c_api::ffi::ncvisual_options;

/// # Constructors
impl<'ncplane> NcVisualOptions {
    /// Returns a builder object for `NcVisualOptions`.
    pub fn builder() -> NcVisualOptionsBuilder<'ncplane> {
        NcVisualOptionsBuilder::default()
    }

    /// New `NcVisualOptions`.
    ///
    /// # Arguments
    ///
    /// * `plane` - an optional mutable pointer to an [`NcPlane`].
    ///
    /// * `scale` - An [`NcScale`] indicating how the source will be
    ///   stretched/scaled relative to the `NcPlane`.
    ///
    /// * `y` - if an `NcPlane` is provided in `plane` then this specifies where
    ///   the `NcVisual` will be on that plane in the *y* axis.
    ///
    ///   Otherwise it specifies where the created `NcPlane` will be placed
    ///   in the *y* axis, relative to the standard plane's origin.
    ///
    ///   If [`VerAligned`] is set, this will be interpreted as an [`NcAlign`]
    ///   value.
    ///
    /// * `x` - if an `NcPlane` is provided in `plane` then this specifies where
    ///   the `NcVisual` will be on that plane, in the *x* axis.
    ///
    ///   Otherwise it specifies where the created `NcPlane` will be placed,
    ///   in the *y* axis, relative to the standard plane's origin.
    ///
    ///   If [`HorAligned`] is set, this will be interpreted as an [`NcAlign`]
    ///   value.
    ///
    /// * `section_yx_lenyx` - The size of the rendered section.
    ///
    ///   `None` renders the entire visual, otherwise the provided tuple
    ///   (`y`, `x`, `len_y`, `len_x`) sets `[yx]` as the origin of the section
    ///   and `len_[yx]` as the its length on each respective dimension.
    ///
    /// * `cell_offset_yx` - Pixel offsets within the cell.
    ///
    ///   If [`NcBlitter::Pixel`] is used the bitmap will be drawn offset from
    ///   the upper-left cell’s origin by these amounts, otherwise this will be
    ///   ignored.
    ///
    ///   It is an error if either number exceeds the cell-pixel geometry in any
    ///   dimension (see [`NcVisualGeometry.cdim_yx`]).
    ///
    /// * `blitter` - [`NcBlitter`] glyph set to use for blitting.
    ///
    /// * `flags` - [`NcVisualFlags`].
    ///
    /// * `transcolor` - treats this color as transparent when the [`AddAlpha`]
    ///   flag is active
    ///
    /// # Notes
    ///
    /// If the [`Childplane`] flag is used then the `plane` is interpreted as
    /// the parent `NcPlane` of the new plane created for this
    /// [`NcVisual`][crate::NcVisual].
    ///
    /// [`NcAlign`]: crate::NcAlign
    /// [`NcPixelGeometry.cell_y`]: crate::NcPixelGeometry#structfield.cell_y
    /// [`NcPixelGeometry.cell_x`]: crate::NcPixelGeometry#structfield.cell_x
    /// [`NcVisualGeometry.cdim_yx`]: crate::NcVisualGeometry#structfield.cdim_yx
    /// [`AddAlpha`]: NcVisualFlags#associatedconstant.AddAlpha
    /// [`Childplane`]: NcVisualFlags#associatedconstant.Childplane
    /// [`VerAligned`]:NcVisualFlags#associatedconstant.VerALigned
    /// [`HorAligned`]: NcVisualFlags#associatedconstant.HorALigned
    pub fn new<RGBA: Into<NcRgba>>(
        plane: Option<&mut NcPlane>,
        scale: NcScale,
        y: i32,
        x: i32,
        section_yx_lenyx: Option<(u32, u32, u32, u32)>,
        cell_offset_yx: Option<(u32, u32)>,
        blitter: NcBlitter,
        flags: NcVisualFlags,
        transcolor: RGBA,
    ) -> Self {
        let plane_ptr = if let Some(p) = plane { p } else { null_mut() };
        let (begy, begx, leny, lenx) =
            if let Some(s) = section_yx_lenyx { (s.0, s.1, s.2, s.3) } else { (0, 0, 0, 0) };
        let (pxoffy, pxoffx) = if let Some(o) = cell_offset_yx { (o.0, o.1) } else { (0, 0) };

        Self {
            n: plane_ptr,
            scaling: scale.into(),

            y,
            x,

            begy,
            begx,
            leny,
            lenx,

            blitter: blitter.into(),

            flags: flags.into(),

            transcolor: transcolor.into().into(),

            pxoffy,
            pxoffx,
        }
    }
}

/// A bitmask of flags for [`NcVisualOptions`].
///
/// # Flags
/// - [`None`][NcVisualFlags::None]
/// - [`AddAlpha`][NcVisualFlags::AddAlpha]
/// - [`Blend`][NcVisualFlags::Blend]
/// - [`ChildPlane`][NcVisualFlags::ChildPlane]
/// - [`NoDegrade`][NcVisualFlags::NoDegrade]
/// - [`HorAligned`][NcVisualFlags::HorAligned]
/// - [`VerAligned`][NcVisualFlags::VerAligned]
/// - [`NoInterpolate`][NcVisualFlags::NoInterpolate]
///
/// # Default
/// *[`NcVisualFlags::None`]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NcVisualFlags(pub c_api::NcVisualFlags_u64);

impl NcVisualFlags {
    /// No flags.
    pub const None: Self = Self(0);

    /// Treats as transparent the color specified in the `transcolor` field.
    pub const AddAlpha: Self = Self(c_api::NCVISUAL_OPTION_ADDALPHA);

    /// Uses [`NcAlpha::Blend`] with the `NcVisual`.
    ///
    /// [`NcAlpha::Blend`]: crate::NcAlpha#associatedconstant.Blend
    pub const Blend: Self = Self(c_api::NCVISUAL_OPTION_BLEND);

    /// allows you to indicate that the n field of ncvisual_options refers not to
    /// the plane onto which you'd like to blit, but the parent of a new plane.
    ///
    /// A plane will be created using the other parameters in the ncvisual_options,
    /// as a child of this parent. This means things like, say, vertically centering
    /// a sprixel relative to the standard plane can be done in one step.
    pub const ChildPlane: Self = Self(c_api::NCVISUAL_OPTION_CHILDPLANE);

    /// Fails rather than gracefully degrade. See [`NcBlitter`][crate::NcBlitter].
    pub const NoDegrade: Self = Self(c_api::NCVISUAL_OPTION_NODEGRADE);

    /// Y is an alignment, not absolute.
    pub const VerAligned: Self = Self(c_api::NCVISUAL_OPTION_VERALIGNED);

    /// X is an alignment, not absolute.
    pub const HorAligned: Self = Self(c_api::NCVISUAL_OPTION_HORALIGNED);

    /// Uses non-interpolative scaling.
    pub const NoInterpolate: Self = Self(c_api::NCVISUAL_OPTION_NOINTERPOLATE);
}

mod std_impls {
    use super::{c_api::NcVisualFlags_u64, NcVisualFlags};

    impl Default for NcVisualFlags {
        fn default() -> Self {
            Self::None
        }
    }

    crate::from_primitive![NcVisualFlags, NcVisualFlags_u64];
    crate::unit_impl_from![NcVisualFlags, NcVisualFlags_u64];
    crate::unit_impl_ops![bitwise; NcVisualFlags, NcVisualFlags_u64];
    crate::unit_impl_fmt![bases+display; NcVisualFlags];
}

pub(crate) mod c_api {
    use super::ffi;

    pub type NcVisualFlags_u64 = u64;

    /// [`NcVisualFlags_u64`] flag to treat as transparent the color specified
    /// in the `transcolor` field.
    pub const NCVISUAL_OPTION_ADDALPHA: NcVisualFlags_u64 =
        ffi::NCVISUAL_OPTION_ADDALPHA as NcVisualFlags_u64;

    /// [`NcVisualFlags_u64`] flag uses [`NcAlpha::Blend`] with the `NcVisual`.
    ///
    /// [`NcAlpha::Blend`]: crate::NcAlpha#associatedconstant.Blend
    pub const NCVISUAL_OPTION_BLEND: NcVisualFlags_u64 =
        ffi::NCVISUAL_OPTION_BLEND as NcVisualFlags_u64;

    /// [`NcVisualFlags_u64`] flag to indicate that the `n` field of
    /// `ncvisual_options` refers not to the plane onto which you'd like to blit,
    /// but the parent of a new plane.
    ///
    /// A plane will be created using the other parameters in the ncvisual_options,
    /// as a child of this parent. This means things like, say, vertically centering
    /// a sprixel relative to the standard plane can be done in one step.
    pub const NCVISUAL_OPTION_CHILDPLANE: NcVisualFlags_u64 =
        ffi::NCVISUAL_OPTION_CHILDPLANE as NcVisualFlags_u64;

    /// [`NcVisualFlags_u64`] flag to fail rather than gracefully degrade.
    ///
    /// See [`NcBlitter`][crate::NcBlitter].
    pub const NCVISUAL_OPTION_NODEGRADE: NcVisualFlags_u64 =
        ffi::NCVISUAL_OPTION_NODEGRADE as NcVisualFlags_u64;

    /// [`NcVisualFlags_u64`] flag to indicate Y is an alignment, not absolute.
    pub const NCVISUAL_OPTION_VERALIGNED: NcVisualFlags_u64 =
        ffi::NCVISUAL_OPTION_VERALIGNED as NcVisualFlags_u64;

    /// [`NcVisualFlags_u64`] flag to indicate X is an alignment, not absolute.
    pub const NCVISUAL_OPTION_HORALIGNED: NcVisualFlags_u64 =
        ffi::NCVISUAL_OPTION_HORALIGNED as NcVisualFlags_u64;

    /// [`NcVisualFlags_u64`] flag to use non-interpolative scaling.
    pub const NCVISUAL_OPTION_NOINTERPOLATE: NcVisualFlags_u64 =
        ffi::NCVISUAL_OPTION_NOINTERPOLATE as NcVisualFlags_u64;
}
