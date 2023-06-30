use crate::ast::node::{self, *};
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

pub fn get_var(n: Node) -> (Token, Token) {
    if let Node::VarDecl(name, type_spec) = n {
        return (name, type_spec);
    } else {
        panic!("Not a var decl node: {}", n);
    }
}
