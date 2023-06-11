use std::collections::HashMap;

use super::node::Node::{self, *};
use super::result::{
    Number::{self, *},
    *,
};
use super::Visit;
use crate::lexer::Token;

#[derive(Debug)]
pub struct Visitor {
    global_scope: HashMap<String, Number>,
}

impl Visitor {
    pub fn new() -> Self {
        Visitor {
            global_scope: HashMap::new(),
        }
    }
}

impl Visit for Visitor {
    fn visit_program(&mut self, _name: String, block: Box<Node>) -> Number {
        self.visit(block)
    }

    fn visit_block(&mut self, var_decls: Vec<Box<Node>>, states: Box<Node>) -> Number {
        for var_decl in var_decls {
            self.visit(var_decl);
        }
        self.visit(states)
    }

    fn visit_var_decl(&mut self, _var_name: Token, _type_spec: Token) -> Number {
        Nil
    }

    fn visit_procedure_decl(&mut self, name: String, block: Box<Node>) -> Number {
        Nil
    }

    fn visit_binop(&mut self, l: Box<Node>, op: Token, r: Box<Node>) -> Number {
        let left = self.visit(l);
        let right = self.visit(r);
        return match op {
            Token::Plus => left + right,
            Token::Minus => left - right,
            Token::Multi => left * right,
            Token::Div => left / right,
            Token::FloatDiv => real_div(left, right),
            _ => panic!("Unrecognized operation: {}", op),
        };
    }
    fn visit_unaryop(&mut self, op: Token, rhs: Box<Node>) -> Number {
        match op {
            Token::Plus => self.visit(rhs),
            Token::Minus => -self.visit(rhs),
            _ => panic!("Unexpected unary operator {}", op),
        }
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
    fn visit_assign(&mut self, lhs: Box<Node>, _: Token, rhs: Box<Node>) -> Number {
        return match *lhs {
            Var(Token::ID(id)) => {
                let value = self.visit(rhs);
                self.global_scope.insert(id, value);
                Nil
            }
            default => panic!(
                "Left hand side of assign statement should be an id, got {}",
                default
            ),
        };
    }
    fn visit_var(&mut self, id: Token) -> Number {
        return match id {
            Token::ID(var_name) => match self.global_scope.get(&var_name) {
                Some(val) => val.clone(),
                None => panic!("Fetch unknown variable from global scope, {}", var_name),
            },
            default => panic!("Want ID, got {}", default),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn test_visitor() {
        let text = r#"
        PROGRAM Part10AST;
        VAR
           a, b : INTEGER;
           y    : REAL;
        
        BEGIN {Part10AST}
           a := 2;
           b := 10 * a + 10 * a DIV 4;
           y := 20 / 7 + 3.14;
        END.  {Part10AST}
                "#;
        let mut p = Parser::new(text.into());
        let tree = p.parse();
        let mut v = Visitor::new();
        let res = v.visit(tree);
        assert_eq!(Nil, res);

        let mut expected: HashMap<String, Number> = HashMap::new();
        expected.insert("a".into(), Int(2));
        expected.insert("b".into(), Int(25));
        expected.insert("y".into(), Number::Real(5.997143));
        assert_eq!(expected, v.global_scope);
    }
}
