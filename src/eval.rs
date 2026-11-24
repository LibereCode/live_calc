use crate::{
    parse::{Expr, Op, Parser},
    tokenize::{ParseError, tokenize},
};

pub fn eval(expr: &Expr) -> Result<f64, ParseError> {
    let value = match expr {
        Expr::Num(n) => *n,
        Expr::Neg(inner) => -eval(inner)?,
        Expr::Binary(lhs, op, rhs) => {
            let a = eval(lhs)?;
            let b = eval(rhs)?;
            match op {
                Op::Add => a + b,
                Op::Sub => a - b,
                Op::Mul => a * b,
                Op::Div => {
                    if b == 0.0 {
                        return Err(ParseError::DivisionByZero);
                    }
                    a / b
                }
                Op::Pow => {
                    if a == 0.0 && b < 0.0 {
                        return Err(ParseError::DivisionByZero);
                    }
                    a.powf(b)
                    // Grammar:
                    //   expr    := term (('+' | '-') term)*
                    //   term    := unary (('*' | '/') unary)*
                    //   unary   := '-' unary | power
                    //   power   := primary ('^' unary)?
                    //   primary := NUMBER | '(' expr ')'
                }
            }
        }
    };

    if value.is_finite() {
        Ok(value)
    } else {
        Err(ParseError::NonFinite)
    }
}

pub fn evaluate(input: &str) -> Result<f64, ParseError> {
    let tokens = tokenize(input)?;
    let expr = Parser::new(tokens).parse()?;
    eval(&expr)
}

/// ---------- Tests ----------
#[cfg(test)]
mod tests {

    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < f64::EPSILON
    }

    #[test]
    fn precedence() {
        assert!(approx(evaluate("3 + 4 * 2").unwrap(), 11.0));
        assert!(approx(evaluate("(3 + 4) * 2").unwrap(), 14.0));
        assert!(approx(evaluate("1.5 * (2 + 2) ^ 2").unwrap(), 24.0));
    }

    #[test]
    fn unary_minus() {
        assert!(approx(evaluate("-2 ^ 2").unwrap(), -4.0));
        assert!(approx(evaluate("2 * -3").unwrap(), -6.0));
        assert!(approx(evaluate("2 ^ -1").unwrap(), 0.5));
    }

    #[test]
    fn associativity() {
        assert!(approx(evaluate("10 - 4 - 3").unwrap(), 3.0));
        assert!(approx(evaluate("8 / 2 / 2").unwrap(), 2.0));
        assert!(approx(evaluate("2 ^ 3 ^ 2").unwrap(), 512.0));
    }

    #[test]
    fn syntax_errors() {
        assert!(matches!(evaluate(""), Err(ParseError::UnexpectedEnd)));
        assert!(matches!(evaluate("(1 + 2"), Err(ParseError::UnexpectedEnd)));
        assert!(matches!(
            evaluate("1 2"),
            Err(ParseError::UnexpectedToken(_, _))
        ));
        assert!(matches!(
            evaluate("3 $ 4"),
            Err(ParseError::UnexpectedChar('$', _))
        ));
        assert!(matches!(
            evaluate("1.2.3"),
            Err(ParseError::InvalidNumber(_))
        ));
    }

    #[test]
    fn math_errors() {
        assert!(matches!(
            evaluate("4 / (2 - 2)"),
            Err(ParseError::DivisionByZero)
        ));
        assert!(matches!(
            evaluate("0 ^ -1"),
            Err(ParseError::DivisionByZero)
        ));
        assert!(matches!(evaluate("(-8) ^ 0.5"), Err(ParseError::NonFinite)));
        assert!(matches!(evaluate("10 ^ 400"), Err(ParseError::NonFinite)));
    }

    #[test]
    /// # FIXED:
    /// - [x] Allow $ (a)(b) = a * b $
    #[allow(clippy::float_cmp)]
    fn brackets() {
        assert_eq!(evaluate("(5)(3)").unwrap(), 15.0);
        assert_eq!(evaluate("(1/2)(0.75)").unwrap(), 3.0 / 8.0);
        assert_eq!(
            evaluate("((1-6/2)^3 * (2 + 3))^2").unwrap(),
            (-2.0f64.powf(3.0) * 5.0).powf(2.0) // = 1600
        );
    }
}
