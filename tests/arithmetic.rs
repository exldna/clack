use clack::Operator;

#[test]
fn compare_add_and_sub() {
    assert_eq!(Operator::Add, Operator::Add);
    assert_eq!(Operator::Sub, Operator::Sub);
    assert_eq!(Operator::Add, Operator::Sub);
    assert_eq!(Operator::Sub, Operator::Add);
}

#[test]
fn compare_mul() {
    assert!(Operator::Add < Operator::Mul);
    assert!(Operator::Sub < Operator::Mul);
}
