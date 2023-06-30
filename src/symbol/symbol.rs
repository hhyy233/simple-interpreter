use crate::lexer::Token;
use std::collections::HashMap;
use std::fmt::Display;

pub const GLOBAL: &str = "global";

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
    ProcedureSymbol(String, Vec<Box<Symbol>>),
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

use Symbol::*;

#[derive(Clone, Debug)]
pub struct ScopedSymbolTable {
    pub name: String,
    pub level: i32,
    symbols: HashMap<String, Symbol>,
    pub enclosing_scope: Option<Box<ScopedSymbolTable>>,
}

impl ScopedSymbolTable {
    pub fn new(name: String, level: i32) -> Self {
        let table = ScopedSymbolTable {
            symbols: HashMap::new(),
            name: name,
            level: level,
            enclosing_scope: None,
        };
        table
    }

    pub fn init(&mut self) {
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
        println!("Insert: {}", s);
        match s {
            VarSymbol(ref name, _) => self.symbols.insert(name.to_string(), s),
            ProcedureSymbol(ref name, _) => self.symbols.insert(name.to_string(), s),
            _ => panic!("Invalid symbol {}", s),
        };
    }

    pub fn lookup(&mut self, name: &String) -> Symbol {
        println!("Lookup: {}, scope name: {}", name, self.name);
        match self.symbols.get(name) {
            Some(s) => s.clone(),
            None => match self.enclosing_scope {
                Some(ref mut pre_scope) => pre_scope.lookup(name),
                None => panic!("Symbol not found {}", name),
            },
        }
    }

    pub fn contains(&mut self, name: &String) -> bool {
        return self.symbols.contains_key(name);
    }
}

impl Display for ScopedSymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "========\n")?;
        write!(f, "Scope name : {}\n", self.name)?;
        write!(f, "Scope level: {}\n", self.level)?;
        let mut enclosed: String = "none".to_owned();
        if let Some(ref scope) = self.enclosing_scope {
            enclosed = scope.name.to_owned();
        }
        write!(f, "Enclosing scope: {}\n", enclosed)?;
        write!(f, "--------\n")?;
        for (k, v) in &self.symbols {
            write!(f, "{:7}: {}\n", k, v)?;
        }
        write!(f, "--------\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let st = ScopedSymbolTable::new(GLOBAL.into(), 1);
        println!("{}", st);
    }
}
