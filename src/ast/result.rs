use std::ops::*;

#[derive(PartialEq, Debug, Clone)]
pub enum Number {
    Nil,
    Int(i32),
}

use self::Number::*;

impl Add for Number {
    type Output = Self;
    fn add(self, rhs: Number) -> Self::Output {
        match self {
            Nil => rhs,
            Int(left) => match rhs {
                Nil => self,
                Int(right) => Number::Int(left + right),
            },
        }
    }
}

impl Sub for Number {
    type Output = Self;
    fn sub(self, rhs: Number) -> Self::Output {
        match self {
            Nil => -rhs,
            Int(left) => match rhs {
                Nil => self,
                Int(right) => Number::Int(left - right),
            },
        }
    }
}

impl Neg for Number {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Int(val) => Int(-val),
            default => default,
        }
    }
}

impl Mul for Number {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Nil => rhs,
            Int(left) => match rhs {
                Nil => self,
                Int(right) => Number::Int(left * right),
            },
        }
    }
}

impl Div for Number {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Nil => rhs,
            Int(left) => match rhs {
                Nil => panic!("Nil cannot be dividend"),
                Int(right) => Number::Int(left / right),
            },
        }
    }
}
