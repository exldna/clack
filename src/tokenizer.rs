use crate::arithmetic;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
    Bracket(arithmetic::Bracket),
    Operator(arithmetic::Operator),
    Number(arithmetic::Number),
}

pub trait Tokenizer: Iterator<Item = Token> + Sized {
    fn reverse_polish_notation(self) -> ReversePolishNotation<Self>;
    fn evaluate(self) -> Result<arithmetic::Number, &'static str>;
}

impl<Iter> Tokenizer for Iter
where
    Iter: Iterator<Item = Token>,
{
    fn reverse_polish_notation(self) -> ReversePolishNotation<Self> {
        ReversePolishNotation::new(self)
    }

    fn evaluate(self) -> Result<arithmetic::Number, &'static str>
    where
        Iter: Iterator<Item = Token>,
    {
        let mut stack: Vec<arithmetic::Number> = Vec::new();
        for token in self.reverse_polish_notation() {
            match token {
                Token::Number(argument) => stack.push(argument),
                Token::Operator(operator) => {
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
                _ => return Err("unexpected token"),
            }
        }
        if stack.len() != 1 {
            return Err("invalid expression");
        }
        Ok(stack.pop().unwrap())
    }
}

pub struct ReversePolishNotation<Iter>
where
    Iter: Iterator<Item = Token>,
{
    flow: Peekable<Iter>,
    stack: Vec<Token>,
}

impl<Iter> ReversePolishNotation<Iter>
where
    Iter: Iterator<Item = Token>,
{
    pub fn new(iter: Iter) -> ReversePolishNotation<Iter> {
        ReversePolishNotation {
            flow: iter.peekable(),
            stack: Vec::new(),
        }
    }
}

impl<Iter> Iterator for ReversePolishNotation<Iter>
where
    Iter: Iterator<Item = Token>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(Token::Operator(op)) = self.stack.last() {
            match self.flow.peek() {
                Some(Token::Operator(operator)) => {
                    if op >= operator {
                        return self.stack.pop();
                    }
                }
                Some(Token::Bracket(arithmetic::Bracket::Right)) => {
                    return self.stack.pop();
                }
                _ => (),
            }
        }
        match self.flow.next() {
            Some(Token::Number(number)) => Some(Token::Number(number)),
            Some(Token::Operator(operator)) => {
                self.stack.push(Token::Operator(operator));
                self.next()
            }
            Some(Token::Bracket(arithmetic::Bracket::Left)) => {
                self.stack.push(Token::Bracket(arithmetic::Bracket::Left));
                self.next()
            }
            Some(Token::Bracket(arithmetic::Bracket::Right)) => {
                self.stack.pop(); // pop left bracket
                self.next()
            }
            None => Some(self.stack.pop()?),
        }
    }
}
