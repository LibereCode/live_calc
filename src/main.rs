use std::io;

mod calc;
mod custom_live;
mod parse;

fn main() -> io::Result<()> {
    custom_live::live_calc()?;

    Ok(())
}
