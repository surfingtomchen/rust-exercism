use crate::Error::{DivisionByZero, InvalidWord, StackUnderflow, UnknownWord};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Clone)]
enum Token {
    Integer(Value),
    Add,
    Minus,
    Multiply,
    Divide,
    Dup,
    Drop,
    Swap,
    Over,
    Word { definition_index: usize },
}

#[derive(Clone)]
struct Definition {
    name: String,
    tokens: Vec<Token>,
}

impl Definition {
    pub fn new(name: &str, tokens: Vec<Token>) -> Self {
        Definition {
            name: name.to_ascii_lowercase(),
            tokens,
        }
    }

    pub fn default_definitions() -> Vec<Definition> {
        let mut temp = vec![];
        temp.push(Definition::new("+", vec![Token::Add]));
        temp.push(Definition::new("*", vec![Token::Multiply]));
        temp.push(Definition::new("-", vec![Token::Minus]));
        temp.push(Definition::new("/", vec![Token::Divide]));
        temp.push(Definition::new("dup", vec![Token::Dup]));
        temp.push(Definition::new("drop", vec![Token::Drop]));
        temp.push(Definition::new("swap", vec![Token::Swap]));
        temp.push(Definition::new("over", vec![Token::Over]));
        temp
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

pub struct Forth {
    stack: Vec<Value>,
    definitions: Vec<Definition>,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            stack: vec![],
            definitions: Definition::default_definitions(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut defining = false;
        let mut strings = vec![];
        for str in input.split_whitespace() {
            match str {
                ":" => {
                    defining = true;
                }
                ";" => {
                    defining = false;
                    self.define_word(&strings)?;
                    strings.clear();
                }
                _ => {
                    if !defining {
                        self.eval_tokens(&vec![self.token_from_str(str)?])?;
                    } else {
                        strings.push(str)
                    }
                }
            }
        }

        if defining {
            Err(InvalidWord)
        } else {
            Ok(())
        }
    }

    fn define_word(&mut self, token_str: &[&str]) -> Result {
        match token_str.len() {
            0..=1 => Err(InvalidWord),
            _ => {
                let tokens = token_str
                    .iter()
                    .skip(1)
                    .map(|s| self.token_from_str(s.to_owned()))
                    .collect::<std::result::Result<Vec<_>, Error>>()?;
                if token_str[0].parse::<Value>().is_ok() {
                    return Err(InvalidWord);
                }
                self.definitions.push(Definition::new(token_str[0], tokens));
                Ok(())
            }
        }
    }

    fn token_from_str(&self, s: &str) -> std::result::Result<Token, Error> {
        match s.parse() {
            Ok(value) => Ok(Token::Integer(value)),
            _ => {
                for i in (0..self.definitions.len()).rev() {
                    if self.definitions[i].name == s.to_ascii_lowercase() {
                        return Ok(Token::Word {
                            definition_index: i,
                        });
                    }
                }
                Err(UnknownWord)
            }
        }
    }

    fn eval_tokens(&mut self, tokens: &[Token]) -> Result {
        for t in tokens {
            if self.stack.len() < 1 {
                match t {
                    Token::Drop | Token::Dup => return Err(StackUnderflow),
                    _ => {}
                }
            } else if self.stack.len() < 2 {
                match t {
                    Token::Add
                    | Token::Minus
                    | Token::Multiply
                    | Token::Divide
                    | Token::Swap
                    | Token::Over => return Err(StackUnderflow),
                    _ => {}
                }
            }

            match t {
                Token::Integer(value) => {
                    self.stack.push(*value);
                }
                Token::Add => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(b + a);
                }
                Token::Minus => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(b - a);
                }
                Token::Multiply => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(b * a);
                }
                Token::Divide => {
                    let a = self.stack.pop().unwrap();
                    if a == 0 {
                        return Err(DivisionByZero);
                    }
                    let b = self.stack.pop().unwrap();
                    self.stack.push(b / a);
                }
                Token::Dup => {
                    self.stack.push(self.stack.last().unwrap().to_owned());
                }
                Token::Drop => {
                    self.stack.pop().unwrap();
                }
                Token::Swap => {
                    let a = self.stack.len();
                    self.stack.swap(a - 1, a - 2);
                }
                Token::Over => {
                    let a = self.stack.len();
                    self.stack.push(self.stack[a - 2]);
                }
                Token::Word {
                    definition_index: i,
                } => {
                    if *i >= self.definitions.len() {
                        return Err(UnknownWord);
                    }

                    self.eval_tokens(&self.definitions[*i].clone().tokens)?;
                }
            }
        }

        Ok(())
    }
}
