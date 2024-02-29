use clack::arithmetic::{Number, Operator};

#[test]
fn add_and_sub_should_be_equal() {
    assert_eq!(Operator::Add, Operator::Add);
    assert_eq!(Operator::Add, Operator::Sub);
    assert_eq!(Operator::Sub, Operator::Add);
    assert_eq!(Operator::Sub, Operator::Sub);
}

#[test]
fn mul_div_rem_should_be_equal() {
    assert_eq!(Operator::Mul, Operator::Mul);
    assert_eq!(Operator::Mul, Operator::Div);
    assert_eq!(Operator::Div, Operator::Mul);
    assert_eq!(Operator::Div, Operator::Div);
    assert_eq!(Operator::Rem, Operator::Mul);
    assert_eq!(Operator::Rem, Operator::Div);
}

#[test]
fn compare() {
    assert!(Operator::Add < Operator::Mul);
    assert!(Operator::Sub < Operator::Mul);
    assert!(Operator::Add < Operator::Div);
    assert!(Operator::Sub < Operator::Div);
    assert!(Operator::Add < Operator::Rem);
    assert!(Operator::Sub < Operator::Rem);
}

#[test]
fn add_constraint() {
    let Number::Integer(_) = Number::Integer(1) + Number::Integer(1) else {
        panic!("Adding int and int must be an integer.");
    };
    let Number::Float(_) = Number::Integer(1) + Number::Float(1.0) else {
        panic!("Adding int and float must be an float.");
    };
    let Number::Float(_) = Number::Float(1.0) + Number::Integer(1) else {
        panic!("Adding float and int must be an float.");
    };
    let Number::Float(_) = Number::Float(1.0) + Number::Float(1.0) else {
        panic!("Adding float and float must be an integer.");
    };
}

#[test]
fn sub_constraint() {
    let Number::Integer(_) = Number::Integer(1) - Number::Integer(1) else {
        panic!("Subtracting int and int must be an integer.");
    };
    let Number::Float(_) = Number::Integer(1) - Number::Float(1.0) else {
        panic!("Subtracting int and float must be an float.");
    };
    let Number::Float(_) = Number::Float(1.0) - Number::Integer(1) else {
        panic!("Subtracting float and int must be an float.");
    };
    let Number::Float(_) = Number::Float(1.0) - Number::Float(1.0) else {
        panic!("Subtracting float and float must be an float.");
    };
}

#[test]
fn mul_constraint() {
    let Number::Integer(_) = Number::Integer(1) * Number::Integer(1) else {
        panic!("Multiplication of int and int must be an integer.");
    };
    let Number::Float(_) = Number::Integer(1) * Number::Float(1.0) else {
        panic!("Multiplication of int and float must be an float.");
    };
    let Number::Float(_) = Number::Float(1.0) * Number::Integer(1) else {
        panic!("Multiplication of float and int must be an float.");
    };
    let Number::Float(_) = Number::Float(1.0) * Number::Float(1.0) else {
        panic!("Multiplication of float and float must be an float.");
    };
}

#[test]
fn div_constraint() {
    let Number::Integer(_) = Number::Integer(1) / Number::Integer(1) else {
        panic!("Diversion of int and int must be an integer.");
    };
    let Number::Float(_) = Number::Integer(1) / Number::Float(1.0) else {
        panic!("Diversion of int and float must be an float.");
    };
    let Number::Float(_) = Number::Float(1.0) / Number::Integer(1) else {
        panic!("Diversion of float and int must be an float.");
    };
    let Number::Float(_) = Number::Float(1.0) / Number::Float(1.0) else {
        panic!("Diversion of float and float must be an float.");
    };
    if let Number::Float(n) = Number::Float(1.0) / Number::Float(0.0) {
        if !n.is_infinite() || !n.is_sign_positive() {
            panic!("Diversion of positive by zero must be positive infinity");
        }
    }
    if let Number::Float(n) = Number::Float(-1.0) / Number::Float(0.0) {
        if !n.is_infinite() || !n.is_sign_negative() {
            panic!("Diversion of negative by zero must be negative infinity");
        }
    }
}

#[test]
fn rem_constraint() {
    let Some(_) = Number::Integer(1) % Number::Integer(1) else {
        panic!("Reminder of int and int always allows.");
    };
    let None = Number::Integer(1) % Number::Float(1.0) else {
        panic!("Reminder of int and float is not allowed.");
    };
    let None = Number::Float(1.0) % Number::Integer(1) else {
        panic!("Reminder of float and int is not allowed.");
    };
    let None = Number::Float(1.0) % Number::Float(1.0) else {
        panic!("Reminder of float and float is not allowed.");
    };
}
