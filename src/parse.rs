//! TODO: This module should also be the one that converts raw input string to `RawKind`
use crate::calc::Operator;

#[derive(Debug, Clone)]
/// # TODO:
/// - [ ] `RawKind::Bracket`
/// - [ ] `RawKind::Dot`
pub enum RawKind {
    Bracket(bool), // TODO:
    Digit(u8),
    Dot, // TODO:
    Operator(Operator),
    Other,
}

/// Will push a parsed `RawKind(char)` to the vec `kinds`
pub fn char_to_rawkind(c: char, kinds: &mut Vec<RawKind>) {
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
    } else if c == '.' {
        kinds.push(RawKind::Dot);
    // } else if ['+', '-', '=', '/'].contains(&c) {
    //     kinds.push(RawKind::Operator(c));
    } else {
        kinds.push(RawKind::Other);
    }
}

/// # TODO:
/// - [ ] [?] Prehaps Also create another `parse_calc_kinds()`,
///   and have this return a set of `CalcKind`s
pub fn parse_raw_kinds(kinds: &[RawKind]) -> String {
    let mut buf = String::new();
    for k in kinds {
        match k {
            RawKind::Bracket(b) => buf.push_str(&b.to_string()),
            RawKind::Digit(u) => buf.push_str(&u.to_string()),
            RawKind::Dot => buf.push('.'),
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
