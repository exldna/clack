use crate::arithmetic;
use crate::tokenizer::Token;
use itertools::Itertools;
use std::iter::Peekable;
use std::str::Chars;

trait Collect<Iter>
where
    Iter: Iterator<Item = char>,
{
    /// Consumes tokens from the flow.
    fn collect(flow: &mut Peekable<Iter>) -> Option<Token>;

    fn try_collect_bracket(flow: &mut Peekable<Iter>) -> Option<arithmetic::Bracket>;
    fn try_collect_operator(flow: &mut Peekable<Iter>) -> Option<arithmetic::Operator>;
    fn try_collect_number(flow: &mut Peekable<Iter>) -> Option<arithmetic::Number>;
}

impl<Iter> Collect<Iter> for Token
where
    Iter: Iterator<Item = char>,
{
    fn collect(flow: &mut Peekable<Iter>) -> Option<Token> {
        if let Some(bracket) = Token::try_collect_bracket(flow) {
            return Some(Token::Bracket(bracket));
        }
        if let Some(token) = Token::try_collect_operator(flow) {
            return Some(Token::Operator(token));
        }
        if let Some(number) = Token::try_collect_number(flow) {
            return Some(Token::Number(number));
        }
        None
    }

    fn try_collect_bracket(flow: &mut Peekable<Iter>) -> Option<arithmetic::Bracket>
    where
        Iter: Iterator<Item = char>,
    {
        let result = match flow.peek() {
            Some('(') => Some(arithmetic::Bracket::Left),
            Some(')') => Some(arithmetic::Bracket::Right),
            _ => None,
        };
        if let Some(_) = result {
            flow.next();
        }
        result
    }

    fn try_collect_operator(flow: &mut Peekable<Iter>) -> Option<arithmetic::Operator>
    where
        Iter: Iterator<Item = char>,
    {
        let result = match flow.peek() {
            Some('+') => Some(arithmetic::Operator::Add),
            Some('-') => Some(arithmetic::Operator::Sub),
            Some('*') => Some(arithmetic::Operator::Mul),
            Some('/') => Some(arithmetic::Operator::Div),
            Some('%') => Some(arithmetic::Operator::Rem),
            _ => None,
        };
        if let Some(_) = result {
            flow.next();
        }
        result
    }

    fn try_collect_number(flow: &mut Peekable<Iter>) -> Option<arithmetic::Number>
    where
        Iter: Iterator<Item = char>,
    {
        // TODO: Implement radix literals
        let mut contains_dot = false;
        let number_src = flow
            .peeking_take_while(|s| match s {
                '.' => {
                    let result = contains_dot == false;
                    contains_dot = true;
                    result
                }
                c if c.is_digit(10) => true,
                _ => false,
            })
            .collect::<String>();
        if !contains_dot {
            let value = number_src.parse::<i32>().ok()?;
            Some(arithmetic::Number::Integer(value))
        } else {
            let value = number_src.parse::<f32>().ok()?;
            Some(arithmetic::Number::Float(value))
        }
    }
}

pub struct Parser<Iter>
where
    Iter: Iterator<Item = char>,
{
    flow: Peekable<Iter>,
}

impl<Iter> Parser<Iter>
where
    Iter: Iterator<Item = char>,
{
    pub fn new(iter: Iter) -> Self {
        Parser::<Iter> {
            flow: iter.peekable(),
        }
    }
}

impl<Iter> Iterator for Parser<Iter>
where
    Iter: Iterator<Item = char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // skip spaces
        while let Some(' ') = self.flow.peek() {
            self.flow.next();
        }
        Token::collect(&mut self.flow)
    }
}

/// Tokenize provides a public API for the Parser.
pub trait Tokenize: Iterator<Item = char> + Sized {
    fn tokenize(self) -> Parser<Self>;
}

impl<Iter> Tokenize for Iter
where
    Iter: Iterator<Item = char>,
{
    fn tokenize(self) -> Parser<Iter> {
        Parser::new(self)
    }
}

pub fn tokenize(string: &str) -> Parser<Chars> {
    return Parser::new(string.chars());
}
