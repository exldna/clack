use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
pub enum Bracket {
    Left,
    Right,
}

#[derive(Debug, Eq, Ord)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

type OperatorRank = u8;

impl Operator {
    fn rank(&self) -> OperatorRank {
        match self {
            Operator::Add => 1,
            Operator::Sub => 1,
            Operator::Mul => 2,
            Operator::Div => 2,
            Operator::Rem => 2,
        }
    }

    pub(crate) fn apply(&self, lhs: Number, rhs: Number) -> Option<Number> {
        match self {
            Operator::Add => Some(lhs + rhs),
            Operator::Sub => Some(lhs - rhs),
            Operator::Mul => Some(lhs * rhs),
            Operator::Div => Some(lhs / rhs),
            Operator::Rem => lhs % rhs,
        }
    }
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        self.rank() == other.rank()
    }
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.rank().cmp(&other.rank()))
    }
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Integer(i32),
    Float(f32),
}

impl std::ops::Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(lhs), Number::Integer(rhs)) => Number::Integer(lhs + rhs),
            (Number::Integer(lhs), Number::Float(rhs)) => Number::Float(lhs as f32 + rhs),
            (Number::Float(lhs), Number::Integer(rhs)) => Number::Float(lhs + rhs as f32),
            (Number::Float(lhs), Number::Float(rhs)) => Number::Float(lhs + rhs),
        }
    }
}

impl std::ops::Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(lhs), Number::Integer(rhs)) => Number::Integer(lhs - rhs),
            (Number::Integer(lhs), Number::Float(rhs)) => Number::Float(lhs as f32 - rhs),
            (Number::Float(lhs), Number::Integer(rhs)) => Number::Float(lhs - rhs as f32),
            (Number::Float(lhs), Number::Float(rhs)) => Number::Float(lhs - rhs),
        }
    }
}

impl std::ops::Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(lhs), Number::Integer(rhs)) => Number::Integer(lhs * rhs),
            (Number::Integer(lhs), Number::Float(rhs)) => Number::Float(lhs as f32 * rhs),
            (Number::Float(lhs), Number::Integer(rhs)) => Number::Float(lhs * rhs as f32),
            (Number::Float(lhs), Number::Float(rhs)) => Number::Float(lhs * rhs),
        }
    }
}

impl std::ops::Div for Number {
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(lhs), Number::Integer(rhs)) => Number::Integer(lhs / rhs),
            (Number::Integer(lhs), Number::Float(rhs)) => Number::Float(lhs as f32 / rhs),
            (Number::Float(lhs), Number::Integer(rhs)) => Number::Float(lhs / rhs as f32),
            (Number::Float(lhs), Number::Float(rhs)) => Number::Float(lhs / rhs),
        }
    }
}

impl std::ops::Rem for Number {
    type Output = Option<Number>;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(lhs), Number::Integer(rhs)) => Some(Number::Integer(lhs % rhs)),
            _ => None // Reminder defined only for integers!
        }
    }
}
