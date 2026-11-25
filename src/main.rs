//! **Calc** is slang for "calculator", btw.

use std::io;

use crate::cli::Config;

mod app;
mod cli;
mod custom_live;
mod eval;
mod parse;
mod tokenize;

/// I like keeping `main()` minimal.
///
/// ## overview
/// For the `ratatui` route:
/// `./cli.rs` -> `./app.rs` -> `./tokenize.rs` -> `./parse.rs` -> `./eval.rs` (-> `./app.rs`)
fn main() -> io::Result<()> {
    let cfg = Config { debug: true };

    cli::run(&cfg)?;

    Ok(())
}
