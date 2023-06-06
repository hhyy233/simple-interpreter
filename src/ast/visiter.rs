use super::node::Node::{self, *};
use crate::lexer::Token::{self, *};

#[derive(Debug)]
pub struct Visitor {}

impl Visitor {
    pub fn new() -> Self {
        Visitor {}
    }

    pub fn visit(&mut self, node: Box<Node>) -> i32 {
        return match *node {
            Num(val) => val,
            BinOp(lhs, op, rhs) => self.visit_binop(lhs, op, rhs),
        };
    }

    fn visit_binop(&mut self, l: Box<Node>, op: Token, r: Box<Node>) -> i32 {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn test_visitor() {
        let text = "3 + 21 * 1 - 7 * 2 - (4 + 6)";
        let mut p = Parser::new(text.into());
        let node = p.parse();
        let mut v = Visitor::new();
        let res = v.visit(node);
        assert_eq!(0, res)
    }
}
