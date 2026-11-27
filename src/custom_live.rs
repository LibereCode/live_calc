//! Using `crossterms::terminal::enable_raw_mode`
//! to do stuff based on _raw-input-events_.

use std::io;

use crossterm::{
    self, cursor,
    event::{self, Event, KeyCode},
    terminal::{self},
};

use crate::{
    eval::EvalParse,
    parse::ParseTokens,
    tokenize::{ParseError, Token, TokensToString, tokenize},
};

pub enum Mode {
    Normal,
    Insert,
}

pub const CLEAR_LINE: &str = "\x1b[2K";

/// Most variables and the while loops are meant for formatting,
/// where they create newlines when a line >= terminal width.
///
/// # TODO:
/// - [ ] [?] Instead of full line reprint, just change print one char.
/// - [ ] Fix this boggy print so it works for 4+ lines
///     - Or just collapse line when it is too long?
fn redraw_line(mode_symbol: &str, raw: &str, parsed: &str) {
    let mut raw_line = String::new() + mode_symbol + " > " + raw;
    let mut parsed_line = String::new() + "    " + parsed;
    let mut parsed_len = parsed_line.len();
    let mut raw_len = raw_line.len();
    let mut parsed_lines = 1;
    let mut raw_lines = 1;
    let clear_new_line = format!("\n\r{CLEAR_LINE}");
    let (col, _row) = terminal::size().unwrap_or_default();
    let col = col as usize;
    while parsed_len >= col {
        parsed_line.insert_str(parsed_lines as usize * col - 1, &clear_new_line);
        parsed_lines += 1;
        parsed_len -= col;
    }
    while raw_len >= col {
        raw_line.insert_str(raw_lines as usize * col - 1, &clear_new_line);
        raw_lines += 1;
        raw_len -= col;
    }
    println!("\r\n\n{CLEAR_LINE}{parsed_line}");
    println!(
        "{}\r{CLEAR_LINE}{raw_line}{}",
        cursor::MoveUp(raw_lines + parsed_lines + 1),
        cursor::MoveUp(1),
    );
}

/// TODO:
/// - [ ] On Enter:
///     - Push calculation to history as a tuple: `(calc_body, sum)`
///     - Should also allow scrolling history with <CTRL-p> / <CTLR-n>
pub fn live_calc() -> io::Result<()> {
    terminal::enable_raw_mode()?;

    let mut mode = Mode::Insert;
    let mut mode_symbol = "";
    let mut raw = String::new();
    let mut message = String::new();
    let mut tokens: Result<Vec<Token>, ParseError>;
    let mut eval: Result<String, String> = Ok(String::new());

    println!("-----\r");
    redraw_line(mode_symbol, &raw, &message);
    loop {
        if let Event::Key(key) = event::read()? {
            // Exit on <CTRL+c>
            if key.modifiers.contains(event::KeyModifiers::CONTROL)
                && key.code == KeyCode::Char('c')
            {
                break;
            }
            redraw_line(mode_symbol, &raw, &message);
            match mode {
                Mode::Insert => {
                    mode_symbol = "";
                    match key.code {
                        // TODO: all of these should add to a vec of symbols/numbers, then
                        // use a calculate function.
                        //
                        KeyCode::Char(c) => {
                            raw.push(c);
                        }
                        KeyCode::Backspace => _ = raw.pop(),
                        KeyCode::Esc => {
                            mode = Mode::Normal;
                            mode_symbol = ""; // <- to update quicker
                        }
                        KeyCode::Enter => {
                            println!(
                                "{CLEAR_LINE}\n\n{CLEAR_LINE}{}\r{}",
                                cursor::MoveUp(2),
                                eval.unwrap_or_default()
                            );
                        }
                        _ => {}
                    }
                }
                Mode::Normal => {
                    mode_symbol = "";

                    match key.code {
                        KeyCode::Insert | KeyCode::Char('i' | 'a' | 'I' | 'A') => {
                            mode = Mode::Insert;
                            mode_symbol = ""; // <- to update quicker
                        }
                        KeyCode::Char('x' | 'X') => _ = raw.pop(),
                        KeyCode::Char('q') => {
                            break;
                        }
                        // TODO: H (J K) L => cursor move
                        _ => {}
                    }
                }
            }

            tokens = tokenize(&raw);
            eval = match &tokens {
                Ok(v) => match v.clone().parse().eval() {
                    Ok(n) => Ok(n.to_string()),
                    Err(e) => Err(e.to_string()),
                },
                Err(e) => Err(e.to_string()),
            };
            message = match &tokens {
                Ok(t) => t.to_string(),
                Err(_) => raw.clone(),
            } + " = "
                + match &eval {
                    Ok(s) | Err(s) => s,
                };
            redraw_line(mode_symbol, &raw, &message);
        }
    }
    println!("\n\n\r\n-----\r");

    terminal::disable_raw_mode()?;
    // println!("{:?}\n{kinds:?}", raw.chars()); // DEBUG
    Ok(())
}
