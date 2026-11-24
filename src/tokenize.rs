use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Num(n) => write!(f, "{n}"),
            Token::Plus => write!(f, "'+'"),
            Token::Minus => write!(f, "'-'"),
            Token::Star => write!(f, "'*'"),
            Token::Slash => write!(f, "'/'"),
            Token::Caret => write!(f, "'^'"),
            Token::LParen => write!(f, "'('"),
            Token::RParen => write!(f, "')'"),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedChar(char, usize),
    InvalidNumber(String),
    UnexpectedToken(String, usize),
    UnexpectedEnd,
    DivisionByZero,
    NonFinite,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedChar(c, i) => {
                write!(f, "unexpected character '{c}' at position {i}")
            }
            ParseError::InvalidNumber(s) => write!(f, "invalid number '{s}'"),
            ParseError::UnexpectedToken(t, i) => {
                write!(f, "unexpected token {t} at position {i}")
            }
            ParseError::UnexpectedEnd => write!(f, "unexpected end of input"),
            ParseError::DivisionByZero => write!(f, "division by zero"),
            ParseError::NonFinite => {
                write!(f, "result is not a finite number (overflow or undefined)")
            }
        }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        match c {
            ' ' | '\t' | '\n' => i += 1,
            '+' => {
                tokens.push(Token::Plus);
                i += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                i += 1;
            }
            '*' => {
                tokens.push(Token::Star);
                i += 1;
            }
            '/' => {
                tokens.push(Token::Slash);
                i += 1;
            }
            '^' => {
                tokens.push(Token::Caret);
                i += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            '0'..='9' | '.' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                let n = s
                    .parse::<f64>()
                    .map_err(|_| ParseError::InvalidNumber(s.clone()))?;
                tokens.push(Token::Num(n));
            }
            other => return Err(ParseError::UnexpectedChar(other, i)),
        }
    }

    Ok(tokens)
}
