use clack::{Bracket, Number, Operator, Token};

#[test]
fn integer() {
    let actual = clack::tokenize("123 4567 12 2 ").collect::<Vec<_>>();
    let expected = vec![
        Token::Number(Number::Integer(123)),
        Token::Number(Number::Integer(4567)),
        Token::Number(Number::Integer(12)),
        Token::Number(Number::Integer(2)),
    ];
    assert_eq!(actual, expected)
}

#[test]
fn match_operators() {
    let actual = clack::tokenize(
        " + + - ++ -+--+ **** +*--"
    ).collect::<Vec<_>>();
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
        Token::Operator(Operator::Mul),
        Token::Operator(Operator::Mul),
        Token::Operator(Operator::Mul),
        Token::Operator(Operator::Add),
        Token::Operator(Operator::Mul),
        Token::Operator(Operator::Sub),
        Token::Operator(Operator::Sub),
    ];
    assert_eq!(actual, expected)
}

#[test]
fn two_plus_two() {
    let actual = clack::tokenize("2 + 2").collect::<Vec<_>>();
    let expected = vec![
        Token::Number(Number::Integer(2)),
        Token::Operator(Operator::Add),
        Token::Number(Number::Integer(2)),
    ];
    assert_eq!(actual, expected)
}

#[test]
fn brackets() {
    let actual = clack::tokenize("(2)").collect::<Vec<_>>();
    let expected = vec![
        Token::Bracket(Bracket::Left),
        Token::Number(Number::Integer(2)),
        Token::Bracket(Bracket::Right),
    ];
    assert_eq!(actual, expected)
}
