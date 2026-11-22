use std::io;

mod app;
mod calc;
mod custom_live;
mod parse;

fn main() -> io::Result<()> {
    // custom_live::live_calc()?;

    ratatui::run(|terminal| app::App::new().run(terminal));

    Ok(())
}
