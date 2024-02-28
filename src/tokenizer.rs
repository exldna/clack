use std::iter::{Peekable};
use std::str::Chars;
use itertools::Itertools;
use crate::arithmetic;
use crate::Token::{Bracket, Number, Operator};

#[derive(Debug, PartialEq)]
pub enum Token {
    Bracket(arithmetic::Bracket),
    Operator(arithmetic::Operator),
    Number(arithmetic::Number),
}

impl Token {
    fn collect<Iter>(flow: &mut Peekable<Iter>) -> Option<Token>
        where Iter: Iterator<Item=char> {
        if let Some(bracket) = Token::collect_bracket(flow) {
            return Some(bracket);
        }
        if let Some(token) = Token::collect_operator(flow) {
            return Some(token);
        }
        if let Some(number) = Token::collect_number(flow) {
            return Some(number);
        }
        None
    }

    fn collect_bracket<Iter>(flow: &mut Peekable<Iter>) -> Option<Token>
        where Iter: Iterator<Item=char> {
        let result = match flow.peek() {
            Some('(') => Some(Bracket(arithmetic::Bracket::Left)),
            Some(')') => Some(Bracket(arithmetic::Bracket::Right)),
            _ => None,
        };
        if let Some(_) = result {
            flow.next();
        }
        result
    }

    fn collect_operator<Iter>(flow: &mut Peekable<Iter>) -> Option<Token>
        where Iter: Iterator<Item=char> {
        let result = match flow.peek() {
            Some('+') => Some(Operator(arithmetic::Operator::Add)),
            Some('-') => Some(Operator(arithmetic::Operator::Sub)),
            Some('*') => Some(Operator(arithmetic::Operator::Mul)),
            Some('/') => Some(Operator(arithmetic::Operator::Div)),
            Some('%') => Some(Operator(arithmetic::Operator::Rem)),
            _ => None,
        };
        if let Some(_) = result {
            flow.next();
        }
        result
    }

    fn collect_number<Iter>(flow: &mut Peekable<Iter>) -> Option<Token>
        where Iter: Iterator<Item=char> {
        let mut contains_dot = false;
        let number_src =
            flow.peeking_take_while(|s| {
                match s {
                    '.' => {
                        let result = contains_dot == false;
                        contains_dot = true;
                        result
                    }
                    c if c.is_digit(10) => true,
                    _ => false,
                }
            }).collect::<String>();
        if !contains_dot {
            let value = number_src.parse::<i32>().ok()?;
            Some(Number(arithmetic::Number::Integer(value)))
        } else {
            let value = number_src.parse::<f32>().ok()?;
            Some(Number(arithmetic::Number::Float(value)))
        }
    }
}

pub trait Tokenize: Iterator<Item=char> + Sized {
    fn tokenize(self) -> Parser<Self>;
}

impl<Iter> Tokenize for Iter
    where Iter: Iterator<Item=char> {
    fn tokenize(self) -> Parser<Iter> {
        Parser::new(self)
    }
}

pub fn tokenize(string: &str) -> Parser<Chars> {
    return Parser::new(string.chars())
}

pub struct Parser<Iter>
    where Iter: Iterator<Item=char> {
    flow: Peekable<Iter>,
}

impl<Iter> Parser<Iter>
    where Iter: Iterator<Item=char> {
    pub fn new(iter: Iter) -> Self {
        Parser {
            flow: iter.peekable()
        }
    }

    fn skip_spaces(&mut self) {
        loop {
            if let Some(' ') = self.flow.peek() {
                self.flow.next();
            } else {
                break;
            }
        }
    }
}

impl<Iter> Iterator for Parser<Iter>
    where Iter: Iterator<Item=char> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_spaces();
        Token::collect(&mut self.flow)
    }
}

pub trait Tokenizer: Iterator<Item=Token> + Sized {
    fn reverse_polish_notation(self) -> ReversePolishNotation<Self>;
    fn evaluate(self) -> Result<arithmetic::Number, &'static str>;
}

impl<Iter> Tokenizer for Iter
    where Iter: Iterator<Item=Token> {
    fn reverse_polish_notation(self) -> ReversePolishNotation<Self> {
        ReversePolishNotation::new(self)
    }

    fn evaluate(self) -> Result<arithmetic::Number, &'static str>
        where Iter: Iterator<Item=Token> {
        let mut stack: Vec<arithmetic::Number> = Vec::new();
        for token in self.reverse_polish_notation() {
            match token {
                Number(argument) => stack.push(argument),
                Operator(operator) => {
                    if let (Some(right), Some(left)) = (stack.pop(), stack.pop()) {
                        if let Some(result) = operator.apply(left, right) {
                            stack.push(result);
                        } else {
                            return Err("cannot apply operator");
                        }
                    } else {
                        return Err("not enough arguments");
                    }
                }
                _ => return Err("unexpected token")
            }
        }
        if stack.len() != 1 {
            return Err("invalid expression");
        }
        return Ok(stack.pop().unwrap());
    }
}

pub struct ReversePolishNotation<Iter>
    where Iter: Iterator<Item=Token> {
    flow: Peekable<Iter>,
    stack: Vec<Token>,
}

impl<Iter> ReversePolishNotation<Iter>
    where Iter: Iterator<Item=Token> {
    pub fn new(iter: Iter) -> ReversePolishNotation<Iter> {
        return ReversePolishNotation {
            flow: iter.peekable(),
            stack: Vec::new(),
        };
    }
}

impl<Iter> Iterator for ReversePolishNotation<Iter>
    where Iter: Iterator<Item=Token> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(Operator(op)) = self.stack.last() {
            match self.flow.peek() {
                Some(Operator(operator)) => {
                    if op >= operator {
                        return self.stack.pop();
                    }
                }
                Some(Bracket(arithmetic::Bracket::Right)) => {
                    return self.stack.pop();
                }
                _ => ()
            }
        }
        match self.flow.next() {
            Some(Number(number)) => Some(Number(number)),
            Some(Operator(operator)) => {
                self.stack.push(Operator(operator));
                self.next()
            }
            Some(Bracket(arithmetic::Bracket::Left)) => {
                self.stack.push(Bracket(arithmetic::Bracket::Left));
                self.next()
            }
            Some(Bracket(arithmetic::Bracket::Right)) => {
                self.stack.pop(); // pop left bracket
                self.next()
            }
            None => Some(self.stack.pop()?)
        }
    }
}
