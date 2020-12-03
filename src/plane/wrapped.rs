//! Handy [`NcPlane`] and [`NcPlaneOptions`] constructors

use core::ptr::{null, null_mut};

// for constructors
use crate::{
    ncpile_create, ncplane_create, notcurses_term_dim_yx, NcAlign, NcPlane, NcPlaneOptions,
    Notcurses, NCPLANE_OPTION_HORALIGNED,
};

// for methods
use crate::{
    NcCell, NcResult, ncplane_cursor_yx, ncplane_dim_yx, ncplane_erase, ncplane_putc, ncplane_putc_yx,
};

/// # `NcPlaneOptions` Constructors
impl NcPlaneOptions {
    /// New NcPlaneOptions using the horizontal x.
    pub fn new(y: i32, x: i32, rows: u32, cols: u32) -> Self {
        Self::with_flags(y, x, rows, cols, 0)
    }

    /// New NcPlaneOptions with horizontal alignment.
    pub fn new_halign(y: i32, align: NcAlign, rows: u32, cols: u32) -> Self {
        Self::with_flags(y, align as i32, rows, cols, NCPLANE_OPTION_HORALIGNED)
    }

    /// New NcPlaneOptions, with flags.
    ///
    /// Note: If you use [NCPLANE_OPTION_HORALIGNED] flag, you must provide
    /// the [NcAlign] value to the `x` parameter, casted to `i32`.
    pub fn with_flags(y: i32, x: i32, rows: u32, cols: u32, flags: u64) -> Self {
        NcPlaneOptions {
            y,
            x,
            rows: rows as i32,
            cols: cols as i32,
            userptr: null_mut(),
            name: null(),
            resizecb: None,
            flags,
        }
    }
}

/// # `NcPlane` Constructors
impl NcPlane {
    /// New NcPlane.
    ///
    /// The returned plane will be the top, bottom, and root of this new pile.
    pub unsafe fn new<'a>(
        nc: &mut Notcurses,
        y: i32,
        x: i32,
        rows: u32,
        cols: u32,
    ) -> &'a mut NcPlane {
        let options = NcPlaneOptions::new(y, x, rows, cols);
        &mut *ncpile_create(nc, &options)
    }

    /// New NcPlane, expects an [NcPlaneOptions] struct.
    ///
    /// The returned plane will be the top, bottom, and root of this new pile.
    pub unsafe fn with_options<'a>(
        nc: &mut Notcurses,
        options: &NcPlaneOptions,
    ) -> &'a mut NcPlane {
        &mut *ncpile_create(nc, options)
    }

    /// New NcPlane, bound to another NcPlane.
    pub unsafe fn new_bound<'a>(
        bound_to: &mut NcPlane,
        y: i32,
        x: i32,
        rows: u32,
        cols: u32,
    ) -> &'a mut NcPlane {
        let options = NcPlaneOptions::new(y, x, rows, cols);
        &mut *ncplane_create(bound_to, &options)
    }

    /// New NcPlane, bound to another plane, expects an [NcPlaneOptions] struct.
    ///
    /// The returned plane will be the top, bottom, and root of this new pile.
    pub unsafe fn with_options_bound<'a>(
        nc: &mut Notcurses,
        options: &NcPlaneOptions,
    ) -> &'a mut NcPlane {
        &mut *ncpile_create(nc, options)
    }

    /// New NcPlane, with the same dimensions of the terminal.
    ///
    /// The returned plane will be the top, bottom, and root of this new pile.
    pub unsafe fn new_termsize<'a>(nc: &mut Notcurses) -> &'a mut NcPlane {
        let (mut trows, mut tcols) = (0, 0);
        notcurses_term_dim_yx(nc, &mut trows, &mut tcols);
        assert![(trows > 0) & (tcols > 0)];
        &mut *ncpile_create(nc, &NcPlaneOptions::new(0, 0, trows as u32, tcols as u32))
    }
}

/// # `NcPlane` Methods
impl NcPlane {
    /// Returns the current position of the cursor within the [NcPlane].
    ///
    /// Unlike [ncplane_cursor_yx] which uses `i32`, this uses [u32].
    //
    // NOTE: y and/or x may be NULL.
    // FIXME: CHECK for NULL and return Some() or None.
    pub fn cursor_yx(&self) -> (u32, u32) {
        let (mut y, mut x) = (0, 0);
        unsafe {ncplane_cursor_yx(self, &mut y, &mut x)};
        (y as u32, x as u32)
    }

    /// Returns the current row of the cursor within the [NcPlane].
    pub fn cursor_y(&self) -> u32 {
        self.cursor_yx().0
    }

    /// Returns the current column of the cursor within the [NcPlane].
    pub fn cursor_x(&self) -> u32 {
        self.cursor_yx().1
    }

    /// Return the dimensions of this [NcPlane].
    ///
    /// Unlike [ncplane_dim_yx] which uses `i32`, this uses [u32].
    pub fn dim_yx(&self) -> (u32, u32) {
        let (mut y, mut x) = (0, 0);
        unsafe {ncplane_dim_yx(self, &mut y, &mut x)};
        (y as u32, x as u32)
    }

    /// Return the rows of this [NcPlane].
    pub fn dim_y(&self) -> u32 {
        self.dim_yx().0
    }

    /// Return the columns of this [NcPlane].
    pub fn dim_x(&self) -> u32 {
        self.dim_yx().1
    }

    /// Erase every NcCell in the NcPlane, resetting all attributes to normal,
    /// all colors to the default color, and all cells to undrawn.
    ///
    /// All cells associated with this ncplane are invalidated, and must not be
    /// used after the call, excluding the base cell. The cursor is homed.
    pub fn erase(&mut self) {
        unsafe { ncplane_erase(self) }
    }

    ///
    pub fn putc_yx(&mut self, y: i32, x: i32, cell: &NcCell) -> NcResult {
        unsafe { ncplane_putc_yx(self, y, x, cell) }
    }
    
    ///
    pub fn putc(&mut self, cell: &NcCell) -> NcResult {
        ncplane_putc(self, cell)
    }
}
