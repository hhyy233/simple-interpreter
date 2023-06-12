use crate::ast::node::Node;
use crate::lexer::Token::*;
use crate::lexer::{Lexer, Token};
use crate::utils::*;

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
}

impl Parser {
    pub fn new(text: String) -> Self {
        let mut l = Lexer::new(text);
        let t = l.get_next_token();
        Parser {
            lexer: l,
            cur_token: t,
        }
    }

    fn get_current_token(&self) -> Token {
        return self.cur_token.clone();
    }

    fn consume(&mut self, tt: &Token) {
        let cur_token = self.get_current_token();
        if cur_token == *tt {
            self.cur_token = self.lexer.get_next_token();
        } else {
            panic!("Unexpected token, expected {}, got {}", tt, cur_token);
        }
    }

    fn factor(&mut self) -> Box<Node> {
        let ct = self.get_current_token();
        return match ct {
            Plus | Minus => {
                self.consume(&ct);
                Box::new(Node::UnaryOp(ct, self.factor()))
            }
            IntConst(ref val) => {
                self.consume(&ct);
                Box::new(Node::Num(get_int(val)))
            }
            RealConst(ref val) => {
                self.consume(&ct);
                Box::new(Node::Real(get_real(val)))
            }
            LParan => {
                self.consume(&LParan);
                let node = self.expr();
                self.consume(&RParan);
                node
            }
            _ => self.variable(),
        };
    }

    fn term(&mut self) -> Box<Node> {
        let mut node = self.factor();
        let mut cur = self.get_current_token();
        while cur == Multi || cur == Div || cur == FloatDiv {
            self.consume(&cur);
            let v = self.factor();
            node = Box::new(Node::BinOp(node, cur, v));
            cur = self.get_current_token()
        }
        node
    }

    fn expr(&mut self) -> Box<Node> {
        let mut node = self.term();
        let mut cur = self.get_current_token();
        while cur == Plus || cur == Minus {
            self.consume(&cur);
            let v = self.term();
            node = Box::new(Node::BinOp(node, cur, v));
            cur = self.get_current_token()
        }
        return node;
    }

    fn block(&mut self) -> Box<Node> {
        let decl_nodes = self.declarations();
        let compound_statement_node = self.compound_statement();
        Box::new(Node::Block(decl_nodes, compound_statement_node))
    }

    fn declarations(&mut self) -> Vec<Box<Node>> {
        let mut decls = vec![];
        if self.get_current_token() == Var {
            self.consume(&Var);
            while let ID(_) = self.get_current_token() {
                let var_decl = self.variable_declaration();
                decls.extend(var_decl);
                self.consume(&Semi);
            }
        }
        while self.get_current_token() == Procedure {
            self.consume(&Procedure);
            let cur_token = self.get_current_token();
            let name = get_id(&cur_token);
            self.consume(&cur_token);
            self.consume(&Semi);
            let block_node = self.block();
            let proc_decl = Node::ProcedureDecl(name, block_node);
            decls.push(Box::new(proc_decl));
            self.consume(&Semi);
        }
        decls
    }

    fn variable_declaration(&mut self) -> Vec<Box<Node>> {
        let mut var_nodes = vec![];
        let cur_token = self.get_current_token();
        if let ID(_) = cur_token {
            self.consume(&cur_token);
            var_nodes.push(cur_token);
        } else {
            return vec![];
        }

        while Comma == self.get_current_token() {
            self.consume(&Comma);
            let cur_token = self.get_current_token();
            if let ID(_) = cur_token {
                self.consume(&cur_token);
                var_nodes.push(cur_token);
            } else {
                panic!("Unexpected token, want ID, got {}", cur_token);
            }
        }

        self.consume(&Colon);
        let type_spec = self.type_spec();

        let mut result = vec![];
        for t in var_nodes {
            result.push(Box::new(Node::VarDecl(t, type_spec.clone())));
        }
        return result;
    }

    fn type_spec(&mut self) -> Token {
        let cur_token = self.get_current_token();
        if cur_token == Integer {
            self.consume(&Integer);
        } else if cur_token == Real {
            self.consume(&Real);
        } else {
            panic!("Unexpected token, want type spec, got {}", cur_token);
        }
        return cur_token;
    }

    fn program(&mut self) -> Box<Node> {
        self.consume(&Program);
        let program_name: String;
        if let Node::Var(ID(name)) = *self.variable() {
            program_name = name;
        } else {
            panic!("Cannot get program name");
        }
        self.consume(&Semi);

        let block = self.block();
        self.consume(&Dot);
        return Box::new(Node::Program(program_name, block));
    }

    fn compound_statement(&mut self) -> Box<Node> {
        self.consume(&Begin);
        let nodes = self.statement_list();
        self.consume(&End);
        Box::new(Node::Compound(nodes))
    }

    fn statement_list(&mut self) -> Vec<Box<Node>> {
        let node = self.statement();
        let mut results = vec![node];
        while self.get_current_token() == Semi {
            self.consume(&Semi);
            results.push(self.statement());
        }
        if let ID(_) = self.get_current_token() {
            panic!("Unexpected id {}", self.get_current_token());
        }
        return results;
    }

    fn statement(&mut self) -> Box<Node> {
        return match self.get_current_token() {
            Begin => self.compound_statement(),
            ID(_) => self.assignment_statement(),
            _ => self.empty(),
        };
    }

    fn assignment_statement(&mut self) -> Box<Node> {
        let left = self.variable();
        self.consume(&Assign);
        let right = self.expr();
        Box::new(Node::Assign(left, Assign, right))
    }

    fn variable(&mut self) -> Box<Node> {
        let cur_token = self.get_current_token();
        return match cur_token {
            ID(_) => {
                self.consume(&cur_token);
                Box::new(Node::Var(cur_token))
            }
            _ => panic!("Unexpected token, want ID, got {}", cur_token),
        };
    }

    fn empty(&mut self) -> Box<Node> {
        return Box::new(Node::NoOp);
    }

    pub fn parse(&mut self) -> Box<Node> {
        let res = self.program();
        let ct = self.get_current_token();
        if ct != EOF {
            panic!("Unexpected token at the end of file, got {}", ct)
        }
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expr() {
        let text = "3 + 21 * 1 + - 7 * 2 - (4 + 6)";
        let mut p = Parser::new(text.into());
        let actual = p.expr();

        let mut node = Box::new(Node::Num(3));
        node = Box::new(Node::BinOp(
            node,
            Plus,
            Box::new(Node::BinOp(
                Box::new(Node::Num(21)),
                Multi,
                Box::new(Node::Num(1)),
            )),
        ));
        node = Box::new(Node::BinOp(
            node,
            Plus,
            Box::new(Node::BinOp(
                Box::new(Node::UnaryOp(Minus, Box::new(Node::Num(7)))),
                Multi,
                Box::new(Node::Num(2)),
            )),
        ));
        node = Box::new(Node::BinOp(
            node,
            Minus,
            Box::new(Node::BinOp(
                Box::new(Node::Num(4)),
                Plus,
                Box::new(Node::Num(6)),
            )),
        ));
        assert_eq!(node, actual);
    }

    #[test]
    #[should_panic]
    fn test_empty() {
        let text = "";
        let mut p = Parser::new(text.into());
        let _ = p.parse();
    }

    #[test]
    fn test_program() {
        let text = r#"
BEGIN
    BEGIN
        number := 2;
        a := number;
        b := 10 * a + 10 * number DIV 4;
        c := a - - b
    END;
    x := 11;
END.
        "#;
        let mut p = Parser::new(text.into());
        let actual = p.compound_statement();
        let expected = Box::new(Node::Compound(vec![
            Box::new(Node::Compound(vec![
                Box::new(Node::Assign(
                    Box::new(Node::Var(ID("number".into()))),
                    Assign,
                    Box::new(Node::Num(2)),
                )),
                Box::new(Node::Assign(
                    Box::new(Node::Var(ID("a".into()))),
                    Assign,
                    Box::new(Node::Var(ID("number".into()))),
                )),
                Box::new(Node::Assign(
                    Box::new(Node::Var(ID("b".into()))),
                    Assign,
                    Box::new(Node::BinOp(
                        Box::new(Node::BinOp(
                            Box::new(Node::Num(10)),
                            Multi,
                            Box::new(Node::Var(ID("a".into()))),
                        )),
                        Plus,
                        Box::new(Node::BinOp(
                            Box::new(Node::BinOp(
                                Box::new(Node::Num(10)),
                                Multi,
                                Box::new(Node::Var(ID("number".into()))),
                            )),
                            Div,
                            Box::new(Node::Num(4)),
                        )),
                    )),
                )),
                Box::new(Node::Assign(
                    Box::new(Node::Var(ID("c".into()))),
                    Assign,
                    Box::new(Node::BinOp(
                        Box::new(Node::Var(ID("a".into()))),
                        Minus,
                        Box::new(Node::UnaryOp(Minus, Box::new(Node::Var(ID("b".into()))))),
                    )),
                )),
            ])),
            Box::new(Node::Assign(
                Box::new(Node::Var(ID("x".into()))),
                Assign,
                Box::new(Node::Num(11)),
            )),
            Box::new(Node::NoOp),
        ]));
        assert_eq!(expected, actual);
    }
}
