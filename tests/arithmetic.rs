use clack::arithmetic::Operator;

#[test]
fn add_and_sub_should_be_equal() {
    assert_eq!(Operator::Add, Operator::Add);
    assert_eq!(Operator::Add, Operator::Sub);
    assert_eq!(Operator::Sub, Operator::Add);
    assert_eq!(Operator::Sub, Operator::Sub);
}

#[test]
fn mul_and_div_should_be_equal() {
    assert_eq!(Operator::Mul, Operator::Mul);
    assert_eq!(Operator::Mul, Operator::Div);
    assert_eq!(Operator::Div, Operator::Mul);
    assert_eq!(Operator::Div, Operator::Div);
}

#[test]
fn compare() {
    assert!(Operator::Add < Operator::Mul);
    assert!(Operator::Sub < Operator::Mul);
    assert!(Operator::Add < Operator::Div);
    assert!(Operator::Sub < Operator::Div);
}
