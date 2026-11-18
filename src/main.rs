use std::io;

mod live;

fn main() -> io::Result<()> {
    live::live_calc()?;
    Ok(())
}
