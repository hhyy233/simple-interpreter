use std::mem;

use super::node::Node::{self};
use super::result::Number::{self, *};
use super::Visit;
use crate::lexer::Token;
use crate::symbol::symbol::*;
use crate::utils::*;

pub struct SemanticAnalyzer {
    cur_scope: Box<ScopedSymbolTable>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut global_scope = ScopedSymbolTable::new(GLOBAL.into(), 1);
        global_scope.init();
        SemanticAnalyzer {
            cur_scope: Box::new(global_scope),
        }
    }
}

impl Visit for SemanticAnalyzer {
    fn visit_block(&mut self, var_decls: Vec<Box<Node>>, states: Box<Node>) -> Number {
        for decl in var_decls {
            self.visit(decl);
        }
        self.visit(states)
    }

    fn visit_program(&mut self, _name: String, block: Box<Node>) -> Number {
        println!("Enter scope: global");
        let res = self.visit(block);
        println!("{}", self.cur_scope);
        return res;
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
        let built_in_type = match self.cur_scope.lookup(&type_spec.to_string()) {
            Symbol::BuiltInSymbol(x) => x,
            unknown => panic!("Unexpected symbol, want Built-in type, got {}", unknown),
        };
        let name = get_id(&var_name);
        if self.cur_scope.contains(&name) {
            panic!("Duplicate id found {}", name)
        }
        let var_symbol = Symbol::VarSymbol(name, built_in_type);
        self.cur_scope.define(var_symbol);
        Nil
    }

    fn visit_procedure_decl(
        &mut self,
        name: String,
        params: Vec<Box<Node>>,
        block: Box<Node>,
    ) -> Number {
        println!("Enter scope: {}", name);

        let level: i32 = self.cur_scope.level + 1;
        let mut procedure_scope = Box::new(ScopedSymbolTable::new(name.to_string(), level));
        // let mut pre_scope = self.cur_scope.clone();
        // self.cur_scope = procedure_scope;

        let mut param_nodes: Vec<Box<Symbol>> = vec![];
        // parse parameters
        for param in params {
            let (var_name, type_spec) = get_var(*param);
            let built_in_type = match self.cur_scope.lookup(&type_spec.to_string()) {
                Symbol::BuiltInSymbol(x) => x,
                unknown => panic!("Unexpected symbol, want Built-in type, got {}", unknown),
            };
            let name = get_id(&var_name);
            if procedure_scope.contains(&name) {
                panic!("Duplicate id found {}", name)
            }
            let var_symbol = Symbol::VarSymbol(name, built_in_type);
            procedure_scope.define(var_symbol.clone());
            param_nodes.push(Box::new(var_symbol.clone()));
        }

        let mut pre_scope = mem::replace(&mut self.cur_scope, procedure_scope);
        let ps = Symbol::ProcedureSymbol(name.to_string(), param_nodes);
        pre_scope.define(ps);
        self.cur_scope.enclosing_scope = Some(pre_scope);

        // parse block
        self.visit(block);

        // inspect the output
        println!("{}", self.cur_scope);

        let pre_scope = self.cur_scope.enclosing_scope.take();
        // the enclosed scope is a must here
        self.cur_scope = pre_scope.unwrap();

        Nil
    }

    fn visit_assign(&mut self, lhs: Box<Node>, _: Token, rhs: Box<Node>) -> Number {
        self.visit(lhs);
        self.visit(rhs);
        Nil
    }

    fn visit_var(&mut self, id: Token) -> Number {
        if let Token::ID(name) = id {
            self.cur_scope.lookup(&name);
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
        let mut s = SemanticAnalyzer::new();
        s.visit(tree);
        let type_spec = s.cur_scope.lookup(&"a".into());
        assert_eq!(
            type_spec,
            Symbol::VarSymbol("a".into(), BuiltIn::new(Token::Integer))
        );
    }

    #[test]
    fn test_scope() {
        let text = r#"
program Main;
    var x, y: real;

    procedure Alpha(a : integer);
        var y : integer;
    begin
        x := a + x + y;
    end;

begin { Main }

end.  { Main }
                "#;
        let mut p = Parser::new(text.into());
        let tree = p.parse();
        let mut s = SemanticAnalyzer::new();
        s.visit(tree);
    }
}
