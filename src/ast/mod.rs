// use enum to implement abstract syntax tree
pub mod node;
mod result;
pub mod symbol;
pub mod visiter;

use crate::lexer::Token;
use node::Node::{self, *};
use result::*;

pub trait Visit {
    fn visit(&mut self, node: Box<Node>) -> Number {
        match *node {
            Program(name, block) => self.visit_program(name, block),
            Block(var_decls, states) => self.visit_block(var_decls, states),
            VarDecl(var_name, var_type) => self.visit_var_decl(var_name, var_type),
            ProcedureDecl(name, block_node) => self.visit_procedure_decl(name, block_node),
            Num(val) => Number::Int(val),
            Node::Real(val) => Number::Real(val),
            BinOp(lhs, op, rhs) => self.visit_binop(lhs, op, rhs),
            UnaryOp(op, rhs) => self.visit_unaryop(op, rhs),
            Compound(nodes) => self.visit_compound(nodes),
            Node::Assign(lhs, op, rhs) => self.visit_assign(lhs, op, rhs),
            Var(id) => self.visit_var(id),
            NoOp => self.visit_noop(),
        }
    }
    fn visit_program(&mut self, name: String, block: Box<Node>) -> Number;
    fn visit_block(&mut self, var_decls: Vec<Box<Node>>, states: Box<Node>) -> Number;
    fn visit_var_decl(&mut self, var_name: Token, type_spec: Token) -> Number;
    fn visit_procedure_decl(&mut self, name: String, block: Box<Node>) -> Number;
    fn visit_binop(&mut self, l: Box<Node>, op: Token, r: Box<Node>) -> Number;
    fn visit_unaryop(&mut self, op: Token, rhs: Box<Node>) -> Number;
    fn visit_compound(&mut self, nodes: Vec<Box<Node>>) -> Number;
    fn visit_noop(&mut self) -> Number;
    fn visit_assign(&mut self, lhs: Box<Node>, _: Token, rhs: Box<Node>) -> Number;
    fn visit_var(&mut self, id: Token) -> Number;
}
