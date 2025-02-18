//! `pixel-cell` example.
//!

#[cfg(not(doc))]
use rand::{distributions::Uniform, Rng};

#[cfg(any(test, doc))]
use crate::*;
#[cfg(not(any(test, doc)))]
use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let mut nc = unsafe { Nc::new()? };

    if nc.check_pixel_support() == NcPixelImpl::None {
        unsafe { nc.stop()? };
        return Err(NcError::new_msg("Current terminal doesn't support pixels."));
    }

    let mut splane = unsafe { nc.stdplane() };
    splane.set_scrolling(true);

    let pg = splane.pixel_geom();

    // print visual delimiters around our pixelized cell
    putstrln!(splane, "0▗│▖\n│─ ─\n2▝│▘")?;
    putstrln!(splane, "a cell is {}x{} pixels", pg.cell_y, pg.cell_x)?;
    putstrln!(
        splane,
        "\ninterpolated  not-interpolated  not-interpolated  interpolated"
    )?;
    putstrln!(
        splane,
        "   SCALE          SCALE               RESIZE          RESIZE"
    )?;
    nc_render_sleep![nc, 1];

    // fill the buffer with random color pixels
    let mut rng = rand::thread_rng();
    let range = Uniform::from(50..=180);
    let mut buffer = Vec::<u8>::with_capacity((pg.cell_y * pg.cell_x * 4) as usize);
    #[allow(unused_parens)]
    for _byte in (0..={ pg.cell_y * pg.cell_x }) {
        buffer.push(rng.sample(&range));
        buffer.push(rng.sample(&range));
        buffer.push(rng.sample(&range));
        buffer.push(255);
    }

    // show the newly created ncvisual delimited with the box drawing characters
    let v1 = NcVisual::from_rgba(buffer.as_slice(), pg.cell_y, pg.cell_x * 4, pg.cell_x)?;
    let voptions = NcVisualOptions::builder().yx(1, 2).pixel().build();
    let p1 = unsafe { v1.blit(&mut nc, Some(&voptions))? };
    p1.reparent(splane)?;

    nc_render_sleep![&mut nc, 1];

    // show the ncvisual, scaled with interpolated values
    // let vplane2 = NcPlane::new_child(&mut splane, 7, 4, 5, 4)?; // TEMP WIP:
    let vplane2opts = NcPlaneOptions::builder().yx(7, 4).rows_cols(5, 4).build();
    let vplane2 = NcPlane::new_child(&mut splane, &vplane2opts)?;
    let voptions2 = NcVisualOptions::builder()
        .plane(vplane2)
        .scale(NcScale::Scale)
        .pixel()
        .build();
    unsafe { v1.blit(&mut nc, Some(&voptions2))? };
    nc_render_sleep![&mut nc, 0, 250];

    // show the ncvisual, scaled without using interpolation
    //let vplane3opts = NcPlaneOptions::builder().yx(7, 19).rows_cols(5, 4).build();
    let vplane3opts = vplane2opts.to_builder().x(19).build();
    let vplane3 = NcPlane::new_child(&mut splane, &vplane3opts)?;
    // let vplane3 = NcPlane::new_child_sized(&mut splane, 7, 19, 5, 4)?;
    // let vplane3 = NcPlane::builder().parent(splane).yx(7, 19).rows_cols(5, 4).build()?;
    let voptions3 = NcVisualOptions::builder()
        .plane(vplane3)
        .scale(NcScale::Scale)
        .pixel()
        .interpolate(false)
        .build();
    unsafe { v1.blit(&mut nc, Some(&voptions3))? };
    nc_render_sleep![&mut nc, 0, 250];

    // resize the ncvisual (doesn't use interpolation)
    let voptions4 = NcVisualOptions::builder().yx(7, 39).pixel().build();
    v1.resize_noninterpolative(pg.cell_y * 4, pg.cell_x * 4)?;
    let p4 = unsafe { v1.blit(&mut nc, Some(&voptions4))? };
    p4.reparent(splane)?;
    nc_render_sleep![&mut nc, 0, 250];

    // resize the ncvisual (uses interpolation)
    let v5 = NcVisual::from_rgba(buffer.as_slice(), pg.cell_y, pg.cell_x * 4, pg.cell_x)?;
    let voptions5 = NcVisualOptions::builder().yx(7, 56).pixel().build();
    v5.resize(pg.cell_y * 4, pg.cell_x * 4)?;
    let p5 = unsafe { v5.blit(&mut nc, Some(&voptions5))? };
    p5.reparent(splane)?;
    nc_render_sleep![&mut nc, 0, 250];

    sleep![2];

    vplane3.destroy()?;
    p4.destroy()?;
    p5.destroy()?;

    v1.destroy();
    v5.destroy();
    unsafe { nc.stop()? };
    Ok(())
}
