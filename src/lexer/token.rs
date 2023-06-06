use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Start,
    Integer,
    IntConst(String),
    RealConst(String),
    ID(String),
    Plus,
    Minus,
    Multi,
    Div,
    Assign,
    LParan,
    RParan,
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
