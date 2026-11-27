//! Replaces old `calc.rs` (together with `./parse.rs`)
//!
//! This module turns `Expr` into `f64`
//! This `f64` is basically the result of the `Expr` $ equation $.
//!
//! # TODO:
//! - [ ] **Factorial** $ a! $
//! - [ ] **Vectors** $ \[a,b,...\] $
//!   Will initially just do a **vec2**
//!     - [ ] _Vector_ **scalar multiplication** $ a * \[b,c,...\] $
//!     - [ ] _Vector_ **vector multiplication** $ \[a, b\] * \[b, c\] $

use crate::{
    parse::{Expr, Op, Parser, PostfixOp, PrefixOp},
    tokenize::ParseError,
};

trait Factorial {
    /// # TODO:
    /// - [ ] Make it handle very big numbers by cheating:
    ///   - If number gets to big, divide it before continue, and
    ///     create a _tuple_ consisting of `(base: i64, exponent: i64)` or something
    fn factorial(self) -> Result<i128, ParseError>
    where
        Self: std::marker::Sized;
}

impl Factorial for u8 {
    fn factorial(self) -> Result<i128, ParseError> {
        let limit = 8_683_317_618_811_886_000_000_000_000_000_000_000; // 33!
        match self {
            0 => Ok(1i128),
            // n if n >= 3 => Err(ParseError::OverflowMultiply(n.to_string())),
            n => Ok(i128::from(n) * {
                let fac = (n - 1).factorial()?;
                if fac >= limit {
                    return Err(ParseError::ToBigFactorial(n as usize, fac.to_string()));
                }
                fac
            }),
        }
    }
}

fn f64_factorial(inner: f64) -> Result<f64, ParseError> {
    if !inner.is_finite() || inner < 0.0 || inner.abs().fract() > f64::EPSILON {
        return Err(ParseError::BadFactorial(inner.to_string()));
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    // Will work, because sign is checked earlier (must be positive), and will fail later if too big
    let n = (inner as u8).factorial()?;

    #[allow(clippy::cast_precision_loss)]
    Ok(n as f64)
}

pub fn eval(expr: &Expr) -> Result<f64, ParseError> {
    let value = match expr {
        Expr::Num(n) => *n,
        Expr::Prefix(op, inner) => match op {
            PrefixOp::Neg => -eval(inner)?,
        },
        Expr::Postfix(inner, op) => match op {
            PostfixOp::Fac => f64_factorial(eval(inner)?)?,
        },
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

pub trait EvalParse {
    /// # Usage
    /// ```rust
    /// // `unwrap_or_default()` works, but there exist better alternatives.
    /// let math_str: &str = "1+1";
    /// let tokens: Vec<Token> = tokenize(math_str).unwrap_or_default();
    /// let parser: Parser = tokens.parse();
    /// let eval: f64 = parser.eval().unwrap_or_default();
    /// ```
    /// ---
    ///
    /// This is an alternative to `evaluate(input: &str)`
    fn eval(&mut self) -> Result<f64, ParseError>;
}

impl EvalParse for Parser {
    fn eval(&mut self) -> Result<f64, ParseError> {
        let expr = self.parse()?;
        eval(&expr)
    }
}

/// ---------- Tests ----------
#[cfg(test)]
mod tests {
    use std::assert_matches;

    use crate::tokenize::tokenize;

    use super::*;

    fn evaluate(input: &str) -> Result<f64, ParseError> {
        let tokens = tokenize(input)?;
        let expr = Parser::new(tokens).parse()?;
        eval(&expr)
    }

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
        assert_matches!(evaluate(""), Err(ParseError::UnexpectedEnd));
        assert_matches!(evaluate("(1 + 2"), Err(ParseError::UnexpectedEnd));
        assert_matches!(evaluate("1 2"), Err(ParseError::UnexpectedToken(_, _)));
        assert_matches!(evaluate("3 $ 4"), Err(ParseError::UnexpectedChar('$', _)));
        assert_matches!(evaluate("1.2.3"), Err(ParseError::InvalidNumber(_)));
    }

    #[test]
    fn math_errors() {
        assert_matches!(evaluate("4 / (2 - 2)"), Err(ParseError::DivisionByZero));
        assert_matches!(evaluate("0 ^ -1"), Err(ParseError::DivisionByZero));
        assert_matches!(evaluate("(-8) ^ 0.5"), Err(ParseError::NonFinite));
        assert_matches!(evaluate("10 ^ 400"), Err(ParseError::NonFinite));
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

    #[test]
    fn factorials() {
        // i64
        assert_matches!(0.factorial(), Ok(1));
        assert_matches!(1.factorial(), Ok(1));
        assert_matches!(4.factorial(), Ok(24));
        assert_matches!(10.factorial(), Ok(3_628_800));

        // f64
        assert_matches!(f64_factorial(0.0), Ok(1.0));
        assert_matches!(f64_factorial(1.0), Ok(1.0));
        assert_matches!(f64_factorial(3.0), Ok(6.0));
        assert_matches!(f64_factorial(7.0), Ok(5_040.0));
    }
}
