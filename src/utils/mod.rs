use crate::lexer::Token::{self, *};

pub fn get_int(v: &String) -> i32 {
    v.parse().unwrap()
}

pub fn get_real(v: &String) -> f32 {
    v.parse().unwrap()
}

pub fn get_id(t: &Token) -> String {
    if let ID(name) = t {
        return name.clone();
    } else {
        panic!("Unexpected token, want ID, got {}", t)
    }
}
