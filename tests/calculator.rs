use clack::arithmetic::Number;
use clack::Config;

#[test]
fn two_plus_two() {
    let actual = clack::calc(Config::query("2 + 2"));
    let expected = Ok(Number::Integer(4));
    assert_eq!(actual, expected)
}

#[test]
fn complex_sum() {
    let actual = clack::calc(Config::query("2 + 2 + 2 + 2"));
    let expected = Ok(Number::Integer(8));
    assert_eq!(actual, expected)
}

#[test]
fn sum_and_sub() {
    let actual = clack::calc(Config::query("2 + 2 - 2 + 2"));
    let expected = Ok(Number::Integer(4));
    assert_eq!(actual, expected)
}

#[test]
fn complex_sum_and_sub() {
    let actual = clack::calc(Config::query("123 + 255 - 122 + 3 - 256"));
    let expected = Ok(Number::Integer(3));
    assert_eq!(actual, expected)
}

#[test]
fn simple_brackets() {
    let actual = clack::calc(Config::query("(2 + 2)"));
    let expected = Ok(Number::Integer(4));
    assert_eq!(actual, expected)
}

#[test]
fn ordering() {
    let actual = clack::calc(Config::query("(2 + 2) * 2"));
    let expected = Ok(Number::Integer(8));
    assert_eq!(actual, expected)
}

#[test]
fn complex_example() {
    let actual = clack::calc(Config::query("10 * (5 + 3) / (8 - 16)"));
    let expected = Ok(Number::Integer(-10));
    assert_eq!(actual, expected)
}

#[test]
fn reminder() {
    let actual = clack::calc(Config::query("3 % 2"));
    let expected = Ok(Number::Integer(1));
    assert_eq!(actual, expected)
}

#[test]
fn bad_reminder() {
    let Err(_) = clack::calc(Config::query("3 % 2.0")) else {
        panic!("Reminder defined only for integers");
    };
}
