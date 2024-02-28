use clack::{Config, Number};

#[test]
fn two_plus_two() {
    let actual = clack::calc(Config::from_str("2 + 2"));
    let expected = Ok(Number::Integer(4));
    assert_eq!(actual, expected)
}

#[test]
fn complex_sum() {
    let actual = clack::calc(Config::from_str("2 + 2 + 2 + 2"));
    let expected = Ok(Number::Integer(8));
    assert_eq!(actual, expected)
}

#[test]
fn sum_and_sub() {
    let actual = clack::calc(Config::from_str("2 + 2 - 2 + 2"));
    let expected = Ok(Number::Integer(4));
    assert_eq!(actual, expected)
}

#[test]
fn complex_sum_sub() {
    let actual = clack::calc(Config::from_str("123 + 255 - 122 + 3 - 256"));
    let expected = Ok(Number::Integer(3));
    assert_eq!(actual, expected)
}

#[test]
fn simple_brackets() {
    let actual = clack::calc(Config::from_str("(2 + 2)"));
    let expected = Ok(Number::Integer(4));
    assert_eq!(actual, expected)
}

#[test]
fn ordering() {
    let actual = clack::calc(Config::from_str("(2 + 2) * 2"));
    let expected = Ok(Number::Integer(8));
    assert_eq!(actual, expected)
}

#[test]
fn complex_sum_sub_mul_div() {
    let actual = clack::calc(
        Config::from_str("10 * (5 + 3) / 8 * 16")
    );
    let expected = Ok(Number::Integer(160));
    assert_eq!(actual, expected)
}

