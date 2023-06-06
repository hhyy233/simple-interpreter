use crate::lexer::Token;
use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Num(i32),
    BinOp(Box<Node>, Token, Box<Node>),
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
