//! Using `crossterms::terminal::enable_raw_mode`
//! to do stuff based on _raw-input-events_.
//!
//! TODO: Move this to it's own project

use crate::{
    calc::{Operator, calculate},
    parse::{RawKind, parse_raw_kinds},
};
use std::io;

use crossterm::{
    self, cursor,
    event::{self, Event, KeyCode},
    terminal,
};

pub enum Mode {
    Normal,
    Insert,
}

pub const CLEAR_LINE: &str = "\x1b[2K";

/// # TODO:
/// - [ ] [?] Instead of full line reprint, just change print one char.
fn redraw_line(mode_symbol: &str, raw: &str, parsed: &str) {
    println!("\r\n\n{CLEAR_LINE}    {parsed}");
    println!(
        "{}\r{CLEAR_LINE}{mode_symbol} > {raw}{}",
        cursor::MoveUp(3),
        cursor::MoveUp(1),
    );
}

fn delete_char(raw: &mut String, kinds: &mut Vec<RawKind>) {
    raw.pop();
    kinds.pop();
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
    let mut parsed = String::new();
    let mut kinds: Vec<RawKind> = Vec::new();
    let mut sum = 0f64.to_string();

    println!("-----\r");
    redraw_line(mode_symbol, &raw, &parsed);
    loop {
        if let Event::Key(key) = event::read()? {
            // Exit on <CTRL+c>
            if key.modifiers.contains(event::KeyModifiers::CONTROL)
                && key.code == KeyCode::Char('c')
            {
                break;
            }
            redraw_line(mode_symbol, &raw, &parsed);
            match mode {
                Mode::Insert => {
                    mode_symbol = "";
                    match key.code {
                        // TODO: all of these should add to a vec of symbols/numbers, then
                        // use a calculate function.
                        KeyCode::Char(c) => {
                            raw.push(c);
                            if c.is_numeric() {
                                kinds.push(RawKind::Digit(c as u8 - b'0'));
                            } else if c == '+' {
                                kinds.push(RawKind::Operator(Operator::Add));
                            } else if c == '-' {
                                kinds.push(RawKind::Operator(Operator::Sub));
                            } else if c == '*' {
                                kinds.push(RawKind::Operator(Operator::Mul));
                            } else if c == '/' {
                                kinds.push(RawKind::Operator(Operator::Div));
                            // } else if ['+', '-', '=', '/'].contains(&c) {
                            //     kinds.push(RawKind::Operator(c));
                            } else {
                                kinds.push(RawKind::Other);
                            }
                        }
                        KeyCode::Backspace => {
                            delete_char(&mut raw, &mut kinds);
                        }
                        KeyCode::Esc => {
                            mode = Mode::Normal;
                            mode_symbol = ""; // <- to update quicker
                        }
                        KeyCode::Enter => {
                            println!("{CLEAR_LINE}\n\n{CLEAR_LINE}{}\r{sum}", cursor::MoveUp(2));
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
                        KeyCode::Char('x' | 'X') => {
                            delete_char(&mut raw, &mut kinds);
                        }
                        KeyCode::Char('q') => {
                            break;
                        }
                        // TODO: H (J K) L => cursor move
                        _ => {}
                    }
                }
            }

            sum = calculate(&kinds);
            parsed = parse_raw_kinds(&kinds);
            parsed.push_str(" = ");
            parsed.push_str(&sum);
            redraw_line(mode_symbol, &raw, &parsed);
        }
    }
    println!("\n\n\r\n-----\r");

    terminal::disable_raw_mode()?;
    println!("{:?}", raw.chars());
    println!("{kinds:?}");
    Ok(())
}
