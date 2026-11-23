use crate::parse::RawKind;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    /// $ a + b $
    Add,
    /// $ a - b $
    Sub,
    /// $ a * b $
    Mul,
    /// $ a / b $
    Div,
    /// $ a^b $
    Exp,
}

impl Operator {
    /// Used in `parse::parse_raw_kinds()` to convert parsed `parse::RawKinds` into _char_
    /// # TODO:
    /// - [ ] Exponent like $ a³ $ instead of $ a ^ 3 $
    pub fn get_sign(&self) -> char {
        match self {
            Operator::Add => '+',
            Operator::Sub => '-',
            Operator::Mul => '*',
            Operator::Div => '/',
            Operator::Exp => '^',
        }
    }
}

/// Iterate over all `RawKind`s, convert them and create a _Vec_ of _tuples_ `(&Operator, f32)`,
/// which will be used by `pedmas()` to get the _result_, and finally convert it to a `String`.
///
/// # TODO:
/// - [x] Parse `RawKind.Dot` (\['.'\], decimals)
/// - [x] Parse _negative_ numbers
/// - [x] BUG where $ --a = ---a = ----a = ... $
/// - [ ] Parse `RawKind.Bracket` (\['(',')'\], for pedmas)
pub fn calculate(kinds: &[RawKind]) -> String {
    // add each number as a pair of the operator before and it's value.
    let mut numbers: Vec<(&Operator, f32)> = Vec::new();
    // first number will be a '+' by default
    let mut operator: &Operator = &Operator::Add;
    let mut current_number = 0f32;
    let mut decimals_len: i32 = 0;

    // Below variable is used to enabling decimal-number input.
    let mut is_decimal = false;

    // 2 variables below enables negative numbers
    let mut is_negative = false;
    let mut digit_after_operator = false;

    for k in kinds {
        match k {
            RawKind::Bracket(_b) => {} // TODO:
            RawKind::Digit(u) => {
                let nr = f32::from(u.to_owned());
                if is_decimal {
                    decimals_len += 1;
                    let new_decimal = nr / 10f32.powi(decimals_len);
                    current_number += new_decimal;
                } else {
                    current_number *= 10f32;
                    current_number += nr;
                }
                // Enables negative numbers, example:
                // $ 8 * -2 $
                if is_negative {
                    current_number = -current_number;
                    is_negative = false;
                }
                digit_after_operator = true;
            }
            RawKind::Dot => {
                is_decimal = true;
            }
            RawKind::Operator(o) => {
                if !digit_after_operator && o == &Operator::Sub {
                    // essentially treat the '-' as $ -a $
                    // instead of like $ a - b $
                    is_negative = !is_negative;
                    continue;
                }
                numbers.push((operator, current_number));
                operator = o;
                current_number = 0f32;
                is_decimal = false;
                digit_after_operator = false;
            }
            RawKind::Other => {}
        }
    }
    numbers.push((operator, current_number));

    pedmas(&numbers).to_string()
}

/// Calculate the result by following **PEDMAS**.
///
/// # FIXED:
/// - [x] **Negative** _exp_/_mul_/_div_ NOT implemented
///   Also current solution (_unwrap or =1_) sucks
///   - [ ] [?] Prehaps need to fix defaults still?
pub fn pedmas(numbers: &[(&Operator, f32)]) -> f64 {
    let mut sum = 0f64;
    let mut local_res: f64;

    // parentheses/brackets (())
    // TODO

    // exponents (^)
    let mut numbers_pe: Vec<(&Operator, f64)> = Vec::new();
    for (o, n) in numbers {
        let n = f64::from(*n);
        let p = match o {
            Operator::Exp => {
                let (prev_o, prev_n) = numbers_pe.pop().unwrap_or((&Operator::Add, 0f64));
                local_res = prev_n.powf(n);
                // local_res = if prev_o == &Operator::Sub {
                //     prev_n.powf(-n)
                // } else {
                //     prev_n.powf(n)
                // };
                (prev_o, local_res)
            }
            _ => (*o, n),
        };
        numbers_pe.push(p);
    }

    // division (/) and multiplication (*)
    let mut numbers_pedm: Vec<(&Operator, f64)> = Vec::new();
    for (o, n) in numbers_pe {
        let p = match o {
            Operator::Mul => {
                let (prev_o, prev_n) = numbers_pedm.pop().unwrap_or((&Operator::Add, 0f64));
                local_res = prev_n * n;
                // local_res = if prev_o == &Operator::Sub {
                //     prev_n * -n
                // } else {
                //     prev_n * n
                // };
                (prev_o, local_res)
            }
            Operator::Div => {
                let (prev_o, prev_n) = numbers_pedm.pop().unwrap_or((&Operator::Add, 0f64));
                local_res = prev_n / n;
                // local_res = if prev_o == &Operator::Sub {
                //     prev_n / -n
                // } else {
                //     prev_n / n
                // };
                (prev_o, local_res)
            }
            _ => (o, n),
        };
        numbers_pedm.push(p);
    }

    // addition (+) and subtraction (-)
    for (o, n) in &numbers_pedm {
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

    sum
}
