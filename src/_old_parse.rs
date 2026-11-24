//! TODO: This module should also be the one that converts raw input string to `RawKind`
use crate::calc::Operator;

#[derive(Debug, Clone)]
/// # TODO:
/// - [ ] `RawKind::Bracket`
/// - [ ] `RawKind::Dot`
pub enum RawKind {
    /// Will be true if inside brackets, ie after opening brackets '(',
    /// and false outside/after closing brackets ')'
    /// # TODO:
    Bracket(bool),
    /// A collection of _digits_ builds a _number_, separated by `calc::Operator`
    Digit(u8),
    /// All _digits_ after the _dot_ in the current _number_ will be interpreted as _decimals_.
    Dot,
    /// `calc::Operator::get_sign()` will convert `calc::Operator` _enum_ into a _char_.
    Operator(Operator),
    /// Will be displayed as a _raw_, but NOT displayed in the _equation_.
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
    } else if c == '^' {
        kinds.push(RawKind::Operator(Operator::Exp));
    } else if c == '.' {
        kinds.push(RawKind::Dot);
    // } else if ['+', '-', '=', '/'].contains(&c) {
    //     kinds.push(RawKind::Operator(c));
    } else {
        kinds.push(RawKind::Other);
    }
}

trait SuperScript {
    /// # TODO:
    /// - [x] Fix parsing for $ 10^0.1 $ (currently '.' is unhandled)
    /// - [ ] Fix parsing for _operators_ and _brackets_, ie: $ 10^(5/4-2*3) $
    fn to_superscript(self) -> String;
}

impl SuperScript for u8 {
    fn to_superscript(self) -> String {
        let superscript = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
        self.to_string()
            .chars()
            .map(|c| superscript[c.to_digit(10).unwrap_or_default() as usize])
            .collect()
    }
}
impl SuperScript for char {
    fn to_superscript(self) -> String {
        match self {
            '.' => '·'.to_string(),
            c => c.to_string(),
        }
    }
}

/// # TODO:
/// - [ ] [?] Prehaps Also create another `parse_calc_kinds()`,
///   and have this return a set of `CalcKind`s
/// - [x] Special case for exponents (_if-statement_ for `Digit` and `Operator`)
pub fn parse_raw_kinds(kinds: &[RawKind]) -> String {
    let mut buf = String::new();
    let mut is_exponent = false;
    let mut number_after_exponent = true;
    for k in kinds {
        match k {
            RawKind::Bracket(b) => buf.push_str(&b.to_string()),
            RawKind::Digit(u) => {
                if is_exponent {
                    buf.push_str(&u.to_superscript());
                } else {
                    buf.push_str(&u.to_string());
                }
                number_after_exponent = true;
            }
            RawKind::Dot => {
                if is_exponent {
                    buf.push_str(&'.'.to_superscript());
                } else {
                    buf.push('.');
                }
            }
            RawKind::Operator(o) => {
                if let Operator::Exp = o {
                    is_exponent = true;
                    number_after_exponent = false;
                } else {
                    // NOTE: TEMPORARY
                    if !number_after_exponent {
                        buf.push('^');
                    }
                    buf.push(' ');
                    buf.push(o.get_sign());
                    buf.push(' ');
                    is_exponent = false;
                }
            }
            RawKind::Other => {}
        }
    }
    buf
}
