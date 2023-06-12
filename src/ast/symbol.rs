use super::node::Node::{self};
use super::result::Number::{self, *};
use super::Visit;
use crate::lexer::Token;
use crate::symbol::symbol::*;
use crate::utils::*;

pub struct SymbolTableBuilder {
    symtab: SymbolTable,
}

impl SymbolTableBuilder {
    pub fn new() -> Self {
        SymbolTableBuilder {
            symtab: SymbolTable::new(),
        }
    }
}

impl Visit for SymbolTableBuilder {
    fn visit_block(&mut self, var_decls: Vec<Box<Node>>, states: Box<Node>) -> Number {
        for decl in var_decls {
            self.visit(decl);
        }
        self.visit(states)
    }

    fn visit_program(&mut self, _name: String, block: Box<Node>) -> Number {
        self.visit(block)
    }

    fn visit_binop(&mut self, l: Box<Node>, _op: Token, r: Box<Node>) -> Number {
        self.visit(l);
        self.visit(r);
        Nil
    }

    fn visit_unaryop(&mut self, _op: Token, rhs: Box<Node>) -> Number {
        self.visit(rhs)
    }

    fn visit_compound(&mut self, nodes: Vec<Box<Node>>) -> Number {
        for child in nodes {
            self.visit(child);
        }
        Nil
    }

    fn visit_noop(&mut self) -> Number {
        Nil
    }

    fn visit_var_decl(&mut self, var_name: Token, type_spec: Token) -> Number {
        let built_in_type = match self.symtab.lookup(&type_spec.to_string()) {
            Symbol::BuiltInSymbol(x) => x,
            unknown => panic!("Unexpected symbol, want Built-in type, got {}", unknown),
        };
        let name = get_id(&var_name);
        if self.symtab.contains(&name) {
            panic!("Duplicate id found {}", name)
        }
        let var_symbol = Symbol::VarSymbol(name, built_in_type);
        self.symtab.define(var_symbol);
        Nil
    }

    fn visit_procedure_decl(&mut self, _name: String, _block: Box<Node>) -> Number {
        Nil
    }

    fn visit_assign(&mut self, lhs: Box<Node>, _: Token, rhs: Box<Node>) -> Number {
        self.visit(lhs);
        self.visit(rhs);
        Nil
    }

    fn visit_var(&mut self, id: Token) -> Number {
        if let Token::ID(name) = id {
            self.symtab.lookup(&name);
        } else {
            panic!("Unexpected token, want ID, got {}", id)
        }
        Nil
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn test_visit() {
        let text = r#"
PROGRAM NameError1;
VAR
   a : INTEGER;

BEGIN
   a := 2 + 1;
END.
        "#;
        let mut p = Parser::new(text.into());
        let tree = p.parse();
        let mut s = SymbolTableBuilder::new();
        s.visit(tree);
        let type_spec = s.symtab.lookup(&"a".into());
        assert_eq!(
            type_spec,
            Symbol::VarSymbol("a".into(), BuiltIn::new(Token::Integer))
        );
    }
}
