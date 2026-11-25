//! _argparse_ and cli-stuff.

use std::{env, io};

use crate::{app, custom_live::live_calc};

#[derive(Debug)]
pub struct Config {
    pub debug: bool,
}

/// # TODO:
/// - [ ] Make some flags overwrite values from `Config`
///   - (--verbose|-v)
pub fn run(cfg: &Config) -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if let Some(a) = args.get(1) {
        if a == "-c" || a == "--custom" {
            live_calc()?;
        } else {
            eprintln!("Unknown argument: {a}");
            std::process::exit(101);
        }
    } else {
        let _rat = ratatui::run(|terminal| app::App::new().run(terminal))?;
        // TODO:
        // - [ ] Should be able to print `Vec<math_new::Token>` (perhaps from `rat.tokens` ?)
        if cfg.debug {
            // println!("{:?}\n{:?}", rat.raw);
        }
    }

    Ok(())
}
