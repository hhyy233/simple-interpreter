use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Token {
    Program,
    Var,
    Begin,
    End,
    Start,
    Integer,
    Real,
    IntConst(String),
    RealConst(String),
    ID(String),
    Plus,
    Minus,
    Multi,
    Div,
    FloatDiv,
    Assign,
    Semi,
    Colon,
    Comma,
    Dot,
    LParan,
    RParan,
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
