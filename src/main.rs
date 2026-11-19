use std::io;

mod calc;
mod live;
mod parse;

fn main() -> io::Result<()> {
    live::live_calc()?;
    Ok(())
}
