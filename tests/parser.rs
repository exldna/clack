use clack::arithmetic::{Bracket, Number, Operator};
use clack::Token;

#[test]
fn int_numbers() {
    let actual = clack::tokenize("123 4567 12 2 ");
    let expected = vec![
        Token::Number(Number::Integer(123)),
        Token::Number(Number::Integer(4567)),
        Token::Number(Number::Integer(12)),
        Token::Number(Number::Integer(2)),
    ];
    assert_eq!(actual.collect::<Vec<_>>(), expected)
}

#[test]
fn float_numbers() {
    let actual = clack::tokenize("12.3 45.67 12.0 0.2 ");
    let expected = vec![
        Token::Number(Number::Float(12.3)),
        Token::Number(Number::Float(45.67)),
        Token::Number(Number::Float(12.0)),
        Token::Number(Number::Float(0.2)),
    ];
    assert_eq!(actual.collect::<Vec<_>>(), expected)
}

#[test]
fn operators() {
    let actual = clack::tokenize(" + + - ++ -+--+ *%%** +*--");
    let expected = vec![
        Token::Operator(Operator::Add),
        Token::Operator(Operator::Add),
        Token::Operator(Operator::Sub),
        Token::Operator(Operator::Add),
        Token::Operator(Operator::Add),
        Token::Operator(Operator::Sub),
        Token::Operator(Operator::Add),
        Token::Operator(Operator::Sub),
        Token::Operator(Operator::Sub),
        Token::Operator(Operator::Add),
        Token::Operator(Operator::Mul),
        Token::Operator(Operator::Rem),
        Token::Operator(Operator::Rem),
        Token::Operator(Operator::Mul),
        Token::Operator(Operator::Mul),
        Token::Operator(Operator::Add),
        Token::Operator(Operator::Mul),
        Token::Operator(Operator::Sub),
        Token::Operator(Operator::Sub),
    ];
    assert_eq!(actual.collect::<Vec<_>>(), expected)
}

#[test]
fn two_plus_two() {
    let actual = clack::tokenize("2 + 2");
    let expected = vec![
        Token::Number(Number::Integer(2)),
        Token::Operator(Operator::Add),
        Token::Number(Number::Integer(2)),
    ];
    assert_eq!(actual.collect::<Vec<_>>(), expected)
}

#[test]
fn simple_brackets() {
    let actual = clack::tokenize("(2)");
    let expected = vec![
        Token::Bracket(Bracket::Left),
        Token::Number(Number::Integer(2)),
        Token::Bracket(Bracket::Right),
    ];
    assert_eq!(actual.collect::<Vec<_>>(), expected)
}

#[test]
fn brackets() {
    let actual = clack::tokenize("(()() ( (() ())) ( )) ");
    let expected = vec![
        Token::Bracket(Bracket::Left),
        Token::Bracket(Bracket::Left),
        Token::Bracket(Bracket::Right),
        Token::Bracket(Bracket::Left),
        Token::Bracket(Bracket::Right),
        Token::Bracket(Bracket::Left),
        Token::Bracket(Bracket::Left),
        Token::Bracket(Bracket::Left),
        Token::Bracket(Bracket::Right),
        Token::Bracket(Bracket::Left),
        Token::Bracket(Bracket::Right),
        Token::Bracket(Bracket::Right),
        Token::Bracket(Bracket::Right),
        Token::Bracket(Bracket::Left),
        Token::Bracket(Bracket::Right),
        Token::Bracket(Bracket::Right),
    ];
    assert_eq!(actual.collect::<Vec<_>>(), expected)
}
