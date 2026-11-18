//! Using `crossterms::terminal::enable_raw_mode`
//! to do stuff based on _raw-input-events_.
//!
//! TODO: Move this to it's own project

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

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn get_sign(&self) -> char {
        match self {
            Operator::Add => '+',
            Operator::Sub => '-',
            Operator::Mul => '*',
            Operator::Div => '/',
        }
    }
}

#[derive(Debug)]
pub enum RawKind {
    Bracket(bool),
    Digit(u8),
    Dot(char),
    Operator(Operator),
    Other,
}

pub const CLEAR_LINE: &str = "\x1b[2K";

// /// TODO:
// pub fn calculate(v: Vec<CalcKind>) {
//     for c in v {
//
//     }
// }

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

/// # TODO:
/// - [ ] Move this to a new **module** (_parse.rs_ ?)
/// - [ ] [?] Prehaps Also create another `parse_calc_kinds()`,
///   and have this return a set of `CalcKind`s
fn parse_raw_kinds(kinds: &[RawKind]) -> String {
    let mut buf = String::new();
    for k in kinds {
        match k {
            RawKind::Bracket(b) => buf.push_str(&b.to_string()),
            RawKind::Digit(u) => buf.push_str(&u.to_string()),
            RawKind::Dot(c) => buf.push(c.to_owned()),
            RawKind::Operator(o) => {
                buf.push(' ');
                buf.push(o.get_sign());
                buf.push(' ');
            }
            RawKind::Other => {}
        }
    }
    buf
}

/// # TODO:
/// - [ ] Move this to a new **module** (_parse.rs_ ?)
/// - [ ] Parse `RawKind.Dot` ('.', decimals)
/// - [ ] Parse `RawKind.Bracket` ('('')', for pedmas)
fn calculate(kinds: &[RawKind]) -> String {
    // add each number as a pair of the operator before and it's value.
    let mut numbers: Vec<(&Operator, f32)> = Vec::new();
    // first number will be a '+' by default
    let mut operator: &Operator = &Operator::Add;
    let mut current_nr = 0f32;
    for k in kinds {
        match k {
            RawKind::Bracket(b) => {}
            RawKind::Digit(u) => {
                let nr = f32::from(u.to_owned());
                current_nr *= 10f32;
                current_nr += nr;
            }
            RawKind::Dot(c) => {}
            RawKind::Operator(o) => {
                numbers.push((operator, current_nr));
                current_nr = 0f32;
                operator = o;
            }
            RawKind::Other => {}
        }
    }
    numbers.push((operator, current_nr));

    pedmas(&numbers).to_string()
}

/// # TODO:
/// - [x] Should follow PEDMAS
/// - [ ] Move this to a new **module** (_parse.rs_ ?)
fn pedmas(numbers: &[(&Operator, f32)]) -> f64 {
    let mut sum = 0f64;
    let mut numbers_pedmas: Vec<(&Operator, f64)> = Vec::new();
    let mut local_res: f64;
    for (o, n) in numbers {
        let n = f64::from(*n);
        let p = match o {
            Operator::Mul => {
                let (prev_o, prev_n) = numbers_pedmas.pop().unwrap_or((&Operator::Add, 1f64));
                local_res = prev_n * n;
                (prev_o, local_res)
            }
            Operator::Div => {
                let (prev_o, prev_n) = numbers_pedmas.pop().unwrap_or((&Operator::Add, 1f64));
                local_res = prev_n / n;
                (prev_o, local_res)
            }
            _ => (*o, n),
        };
        numbers_pedmas.push(p);
    }
    for (o, n) in &numbers_pedmas {
        match o {
            Operator::Add => {
                sum += n;
            }
            Operator::Sub => {
                sum -= n;
            }
            _ => {}
        }
    }
    // TEST DEBUG
    // println!("{numbers:?}");
    // println!("{numbers_pedmas:?}");
    //
    sum
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
    let mut parsed_string;
    let mut parsed = "";
    let mut kinds: Vec<RawKind> = Vec::new();
    let mut sum = 0f64.to_string();

    println!("-----\r");
    redraw_line(mode_symbol, &raw, parsed);
    loop {
        if let Event::Key(key) = event::read()? {
            // Exit on <CTRL+c>
            if key.modifiers.contains(event::KeyModifiers::CONTROL)
                && key.code == KeyCode::Char('c')
            {
                break;
            }
            redraw_line(mode_symbol, &raw, parsed);
            match mode {
                Mode::Insert => {
                    mode_symbol = "";
                    match key.code {
                        // TODO: all of these should add to a vec of symbols/numbers, then
                        // use a calculate function.
                        KeyCode::Char(c) => {
                            raw += c.to_string().as_str();
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

            // for c in raw.chars() {
            //     if c.is_numeric() {
            //         kinds.push(CalcKind::Digit(c.to_digit(10).unwrap_or_default()));
            //     } else if ['+', '-', '=', '/'].contains(&c) {
            //         kinds.push(CalcKind::Operator(c));
            //     }
            // }
            // TODO:
            sum = calculate(&kinds);
            parsed_string = parse_raw_kinds(&kinds);
            parsed_string.push_str(" = ");
            parsed_string.push_str(&sum);
            parsed = parsed_string.as_str();
            redraw_line(mode_symbol, &raw, parsed);
        }
    }
    println!("\n\n\r\n-----\r");

    terminal::disable_raw_mode()?;
    println!("{:?}", raw.chars());
    println!("{kinds:?}");
    Ok(())
}
