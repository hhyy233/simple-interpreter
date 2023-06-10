use crate::lexer::Token;
use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Program(String, Box<Node>),       // variable name, blocks
    Block(Vec<Box<Node>>, Box<Node>), // declarations, compound statement
    VarDecl(Token, Token),            // variable, type token
    Num(i32),
    Real(f32),
    BinOp(Box<Node>, Token, Box<Node>),
    UnaryOp(Token, Box<Node>),           // Plus | Minus, number
    Assign(Box<Node>, Token, Box<Node>), // variable, :=, expression
    Var(Token),                          // identifier
    Compound(Vec<Box<Node>>),
    NoOp,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
