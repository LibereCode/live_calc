use crate::parse::RawKind;

#[derive(Debug, Clone)]
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

/// # TODO:
/// - [ ] Parse `RawKind.Dot` ('.', decimals)
/// - [ ] Parse `RawKind.Bracket` ('('')', for pedmas)
pub fn calculate(kinds: &[RawKind]) -> String {
    // add each number as a pair of the operator before and it's value.
    let mut numbers: Vec<(&Operator, f32)> = Vec::new();
    // first number will be a '+' by default
    let mut operator: &Operator = &Operator::Add;
    let mut current_nr = 0f32;
    for k in kinds {
        match k {
            RawKind::Bracket(_b) => {} // TODO:
            RawKind::Digit(u) => {
                let nr = f32::from(u.to_owned());
                current_nr *= 10f32;
                current_nr += nr;
            }
            RawKind::Dot(_c) => {} // TODO:
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

pub fn pedmas(numbers: &[(&Operator, f32)]) -> f64 {
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

    sum
}
