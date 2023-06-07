use std::collections::HashMap;

use super::node::Node::{self, *};
use super::result::Number::{self, *};
use crate::lexer::Token::{self, *};

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

    pub fn visit(&mut self, node: Box<Node>) -> Number {
        return match *node {
            Num(val) => Number::Int(val),
            BinOp(lhs, op, rhs) => self.visit_binop(lhs, op, rhs),
            UnaryOp(op, rhs) => self.visit_unaryop(op, rhs),
            Compound(nodes) => self.visit_compound(nodes),
            Node::Assign(lhs, op, rhs) => self.visit_assign(lhs, op, rhs),
            Var(id) => self.visit_var(id),
            NoOp => self.visit_noop(),
            _ => todo!(),
        };
    }

    fn visit_binop(&mut self, l: Box<Node>, op: Token, r: Box<Node>) -> Number {
        let left = self.visit(l);
        let right = self.visit(r);
        return match op {
            Plus => left + right,
            Minus => left - right,
            Multi => left * right,
            Div => left / right,
            _ => panic!("Unrecognized operation: {}", op),
        };
    }
    fn visit_unaryop(&mut self, op: Token, rhs: Box<Node>) -> Number {
        match op {
            Plus => self.visit(rhs),
            Minus => -self.visit(rhs),
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
            Var(ID(id)) => {
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
            ID(var_name) => match self.global_scope.get(&var_name) {
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
        BEGIN
            BEGIN
                number := 2;
                a := number;
                b := 10 * a + 10 * number / 4;
                c := a - - b
            END;
            x := 11;
        END.
                "#;
        let mut p = Parser::new(text.into());
        let tree = p.parse();
        let mut v = Visitor::new();
        let res = v.visit(tree);
        assert_eq!(Nil, res);

        let mut expected = HashMap::new();
        expected.insert("a", Int(2));
        expected.insert("x", Int(11));
        expected.insert("c", Int(27));
        expected.insert("b", Int(25));
        expected.insert("number", Int(2));
    }
}
