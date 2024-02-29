use crate::tokenizer::Token;
use std::iter::Peekable;
use std::str::Chars;

// impl Token {
//     fn collect<Iter>(flow: &mut Peekable<Iter>) -> Option<Token>
//         where Iter: Iterator<Item=char> {
//         if let Some(bracket) = Token::collect_bracket(flow) {
//             return Some(bracket);
//         }
//         if let Some(token) = Token::collect_operator(flow) {
//             return Some(token);
//         }
//         if let Some(number) = Token::collect_number(flow) {
//             return Some(number);
//         }
//         None
//     }
//
//     fn collect_bracket<Iter>(flow: &mut Peekable<Iter>) -> Option<Token>
//         where Iter: Iterator<Item=char> {
//         let result = match flow.peek() {
//             Some('(') => Some(Bracket(arithmetic::Bracket::Left)),
//             Some(')') => Some(Bracket(arithmetic::Bracket::Right)),
//             _ => None,
//         };
//         if let Some(_) = result {
//             flow.next();
//         }
//         result
//     }
//
//     fn collect_operator<Iter>(flow: &mut Peekable<Iter>) -> Option<Token>
//         where Iter: Iterator<Item=char> {
//         let result = match flow.peek() {
//             Some('+') => Some(Operator(arithmetic::Operator::Add)),
//             Some('-') => Some(Operator(arithmetic::Operator::Sub)),
//             Some('*') => Some(Operator(arithmetic::Operator::Mul)),
//             Some('/') => Some(Operator(arithmetic::Operator::Div)),
//             Some('%') => Some(Operator(arithmetic::Operator::Rem)),
//             _ => None,
//         };
//         if let Some(_) = result {
//             flow.next();
//         }
//         result
//     }
//
//     fn collect_number<Iter>(flow: &mut Peekable<Iter>) -> Option<Token>
//         where Iter: Iterator<Item=char> {
//         let mut contains_dot = false;
//         let number_src =
//             flow.peeking_take_while(|s| {
//                 match s {
//                     '.' => {
//                         let result = contains_dot == false;
//                         contains_dot = true;
//                         result
//                     }
//                     c if c.is_digit(10) => true,
//                     _ => false,
//                 }
//             }).collect::<String>();
//         if !contains_dot {
//             let value = number_src.parse::<i32>().ok()?;
//             Some(Number(arithmetic::Number::Integer(value)))
//         } else {
//             let value = number_src.parse::<f32>().ok()?;
//             Some(Number(arithmetic::Number::Float(value)))
//         }
//     }
// }

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
        Parser {
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
        None // TODO: parse token from flow
    }
}
