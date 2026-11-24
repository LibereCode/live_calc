use std::{env, io};

mod app;
mod custom_live;
mod eval;
mod parse;
mod tokenize;

fn main() -> io::Result<()> {
    // TODO: make this a flag (--verbose|-v)
    let debug = true;

    let args: Vec<String> = env::args().collect();
    if let Some(a) = args.get(1) {
        if a == "-c" || a == "--custom" {
            custom_live::live_calc()?;
        } else {
            eprintln!("Unknown argument: {a}");
            std::process::exit(101);
        }
    } else {
        let rat = ratatui::run(|terminal| app::App::new().run(terminal))?;
        // TODO:
        // - [ ] Should be able to print `Vec<math_new::Token>` (perhaps from `rat.tokens` ?)
        if debug {
            // println!("{:?}\n{:?}", rat.raw);
        }
    }

    Ok(())
}
