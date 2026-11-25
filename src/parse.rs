//! Replaces old `calc.rs` (together with `./eval.rs`)
//!
//! This module turns `Token`(s) into `Expr`
//! `Expr` is basically the whole $ equation $, or it's parts.

use crate::tokenize::{ParseError, Token};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug)]
pub enum Expr {
    Num(f64),
    Neg(Box<Expr>),
    Binary(Box<Expr>, Op, Box<Expr>),
}

/// Grammar:
///   expr    := term (('+' | '-') term)*
///   term    := unary (('*' | '/') unary)*
///   unary   := '-' unary | power
///   power   := primary ('^' unary)?
///   primary := NUMBER | '(' expr ')'
pub struct Parser {
    pub tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    /// Return current position's token, and advance position by +1.
    fn advance(&mut self) -> Option<Token> {
        let tok = self.peek().cloned();
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let expr = self.expr()?;
        if let Some(tok) = self.peek() {
            return Err(ParseError::UnexpectedToken(tok.to_string(), self.pos));
        }
        Ok(expr)
    }

    /// expr := term (('+' | '-') term)*
    /// 1+ `terms`, joined by +|-
    /// example: $ a + b $
    fn expr(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.term()?;
        loop {
            let op = match self.peek() {
                Some(Token::Plus) => Op::Add,
                Some(Token::Minus) => Op::Sub,
                _ => break,
            };
            // Grammar:
            //   expr    := term (('+' | '-') term)*
            //   term    := unary (('*' | '/') unary)*
            //   unary   := '-' unary | power
            //   power   := primary ('^' unary)?
            //   primary := NUMBER | '(' expr ')'
            self.advance();
            let rhs = self.term()?;
            lhs = Expr::Binary(Box::new(lhs), op, Box::new(rhs));
        }
        Ok(lhs)
    }

    /// term := unary (('*' | '/') unary)*
    /// 1+ `unary`, joined by *|/
    /// example: $ a * b $
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.unary()?;
        loop {
            let tok = self.peek();
            let op = match tok {
                Some(Token::Star | Token::LParen) => Op::Mul,
                Some(Token::Slash) => Op::Div,
                _ => break,
            };
            // treat $ (a)(b) $ as $ (a)*(b) $
            if tok.ne(&Some(&Token::LParen)) {
                self.advance();
            }
            let rhs = self.unary()?;
            lhs = Expr::Binary(Box::new(lhs), op, Box::new(rhs));
        }
        Ok(lhs)
    }

    /// unary := '-' unary | power
    /// (-)? `power`
    /// example: $ a $ | $ -a $ | $ -a^b $ | $ (a-b)^c $
    fn unary(&mut self) -> Result<Expr, ParseError> {
        // if current position is minus, re-run this function and make negative.
        if matches!(self.peek(), Some(Token::Minus)) {
            self.advance();
            let inner = self.unary()?;
            return Ok(Expr::Neg(Box::new(inner)));
        }
        self.power()
    }

    /// power := primary ('^' unary)?
    /// `primary`, followed by (^`unary`)?
    /// example: $ a $  | $ a^b $ | $ (a-b)^c $ | $ -(a+b) $
    fn power(&mut self) -> Result<Expr, ParseError> {
        let base = self.primary()?;
        // if current is a bracket, run `unary()` on the bracket's first item.
        if matches!(self.peek(), Some(Token::Caret)) {
            self.advance();
            let exp = self.unary()?;
            return Ok(Expr::Binary(Box::new(base), Op::Pow, Box::new(exp)));
        }
        Ok(base)
    }

    /// primary := NUMBER | '(' expr ')'
    /// number|parenthesized expression
    fn primary(&mut self) -> Result<Expr, ParseError> {
        match self.advance() {
            Some(Token::Num(n)) => Ok(Expr::Num(n)),
            // if token is a '(', run `expr()` on the next tokens until finding matching ')'.
            Some(Token::LParen) => {
                let inner = self.expr()?;
                match self.advance() {
                    Some(Token::RParen) => Ok(inner),
                    Some(t) => Err(ParseError::UnexpectedToken(t.to_string(), self.pos - 1)),
                    None => Err(ParseError::UnexpectedEnd),
                }
            }
            Some(t) => Err(ParseError::UnexpectedToken(t.to_string(), self.pos - 1)),
            None => Err(ParseError::UnexpectedEnd),
        }
    }
}

pub trait ParseTokens {
    /// # Usage
    /// ```rust
    /// // Please avoid using `unwrap()`, do a better error-handling instead
    /// let math_str: &str = "1+1";
    /// let tokens: Vec<Token> = tokenize(math_str).unwrap_or_default();
    /// let parser: Parser = tokens.parse();
    /// ```
    fn parse(self) -> Parser;
}

impl ParseTokens for Vec<Token> {
    fn parse(self) -> Parser {
        Parser::new(self)
    }
}
