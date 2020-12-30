use libnotcurses_sys::*;

fn main() -> NcResult<()> {
    let nc = Notcurses::new()?;
    let stdplane = nc.stdplane()?;
    let p1 = NcPlane::new(nc, 0, 0, 20, 30);

    for ch in "Initializing cells...".chars() {
        let cell = NcCell::with_char7b(ch);
        stdplane.putc(&cell)?;
        sleep![0, 40];
        nc.render()?;
    }
    sleep![0, 900];

    nc.stop()?;
    Ok(())
}
