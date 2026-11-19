use crate::calc::Operator;

#[derive(Debug)]
/// # TODO:
/// - [ ] `RawKind::Bracket`
/// - [ ] `RawKind::Dot`
pub enum RawKind {
    Bracket(bool), // TODO:
    Digit(u8),
    Dot(char), // TODO:
    Operator(Operator),
    Other,
}

/// # TODO:
/// - [ ] Move this to a new **module** (_parse.rs_ ?)
/// - [ ] [?] Prehaps Also create another `parse_calc_kinds()`,
///   and have this return a set of `CalcKind`s
pub fn parse_raw_kinds(kinds: &[RawKind]) -> String {
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
