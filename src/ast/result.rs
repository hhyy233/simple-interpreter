use std::ops::*;

#[derive(PartialEq, Debug, Clone)]
pub enum Number {
    Nil,
    Int(i32),
    Real(f32),
}

use self::Number::*;

pub fn real_div(lhs: Number, rhs: Number) -> Number {
    let left = get_real(lhs);
    let right = get_real(rhs);
    Real(left / right)
}

fn get_real(num: Number) -> f32 {
    return match num {
        Nil => panic!("Got Nil in division"),
        Int(val) => val as f32,
        Real(val) => val,
    };
}

impl Add for Number {
    type Output = Self;
    fn add(self, rhs: Number) -> Self::Output {
        match self {
            Nil => rhs,
            Int(left) => match rhs {
                Nil => self,
                Int(right) => Number::Int(left + right),
                Real(right) => panic!("Invalid addition, {} + {}", left, right),
            },
            Real(left) => match rhs {
                Nil => self,
                Real(right) => Number::Real(left + right),
                Int(right) => panic!("Invalid addition, {} + {}", left, right),
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
                Real(right) => panic!("Invalid substration, {} + {}", left, right),
            },
            Real(left) => match rhs {
                Nil => self,
                Int(right) => panic!("Invalid addition, {} + {}", left, right),
                Real(right) => Number::Real(left - right),
            },
        }
    }
}

impl Neg for Number {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Nil => Nil,
            Int(val) => Int(-val),
            Real(val) => Real(-val),
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
                Real(right) => panic!("Invalid multiple, {} * {}", left, right),
            },
            Real(left) => match rhs {
                Nil => self,
                Int(right) => panic!("Invalid multiple, {} * {}", left, right),
                Real(right) => Number::Real(left * right),
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
                Real(right) => Number::Int(left / right as i32),
            },
            Real(left) => match rhs {
                Nil => panic!("Nil cannot be dividend"),
                Int(right) => Number::Int(left as i32 / right),
                Real(right) => Number::Int((left / right) as i32),
            },
        }
    }
}
