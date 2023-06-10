use crate::lexer::Token;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct BuiltIn(Token);

impl BuiltIn {
    pub fn new(t: Token) -> Self {
        match t {
            Token::Integer | Token::Real => BuiltIn(t),
            _ => panic!("Invalid built-in type {}", t),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Symbol {
    BuiltInSymbol(BuiltIn),
    VarSymbol(String, BuiltIn),
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

use Symbol::*;

pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = SymbolTable {
            symbols: HashMap::new(),
        };
        table.init();
        table
    }

    fn init(&mut self) {
        let int_type = BuiltIn::new(Token::Integer);
        let real_type = BuiltIn::new(Token::Real);
        self.set(int_type);
        self.set(real_type);
    }

    fn set(&mut self, t: BuiltIn) {
        self.symbols
            .insert(t.0.to_string(), Symbol::BuiltInSymbol(t));
    }

    pub fn define(&mut self, s: Symbol) {
        match s {
            VarSymbol(ref name, _) => self.symbols.insert(name.to_string(), s),
            _ => panic!("Invalid symbol {}", s),
        };
    }

    pub fn lookup(&mut self, name: String) -> Symbol {
        match self.symbols.get(&name) {
            Some(s) => s.clone(),
            None => panic!("Symbol not found {}", name),
        }
    }
}
