use crate::Operator::{Divided, Minus, Multiplied, Plus};

#[derive(Copy, Clone)]
enum Operator {
    Plus,
    Minus,
    Multiplied,
    Divided,
}

impl Operator {
    pub fn calculate(&self, a: i32, b: i32) -> i32 {
        match self {
            Plus => a + b,
            Minus => a - b,
            Multiplied => a * b,
            Divided => a / b,
        }
    }
}

#[derive(Copy, Clone)]
enum Token {
    Number(i32),
    Op(Operator),
    UnknownOp,
}

impl From<i32> for Token {
    fn from(i: i32) -> Self {
        Token::Number(i)
    }
}

impl Token {
    fn new(s: &str) -> Option<Self> {
        if let Ok(number) = s.parse() {
            Some(Token::Number(number))
        } else {
            match s {
                "plus" => Some(Token::Op(Plus)),
                "minus" => Some(Token::Op(Minus)),
                "multiplied" => Some(Token::Op(Multiplied)),
                "divided" => Some(Token::Op(Divided)),
                "power" | "cubed" => Some(Token::UnknownOp),
                _ => None,
            }
        }
    }
}

pub fn answer(command: &str) -> Option<i32> {
    command
        .replace('?', "")
        .split(' ')
        .filter_map(|s| Token::new(s))
        .fold(Some((None, None)), |result, word| {
            result.and_then(|(value, op)| match (value, op, word) {
                (Some(Token::Number(v)), Some(Token::Op(op)), Token::Number(n)) => {
                    Some((Some(Token::from(op.calculate(v, n))), None))
                }
                (None, None, Token::Number(_)) => Some((Some(word), None)),
                (Some(_), None, Token::Op(_)) => Some((value, Some(word))),
                _ => None,
            })
        })
        .and_then(|r| match r {
            (Some(Token::Number(n)), None) => Some(n),
            _ => None,
        })
}
