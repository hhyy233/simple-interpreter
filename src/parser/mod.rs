use crate::ast::node::*;
use crate::lexer::Token::*;
use crate::lexer::{Lexer, Token};

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
            IntConst(_) => {
                self.consume(&ct);
                Box::new(Node::Num(get_int(ct)))
            }
            LParan => {
                self.consume(&LParan);
                let node = self.expr();
                self.consume(&RParan);
                node
            }
            _ => panic!("Unexpected factor {}", ct),
        };
    }
    fn term(&mut self) -> Box<Node> {
        let mut node = self.factor();
        let mut cur = self.get_current_token();
        while cur == Multi || cur == Div {
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
    pub fn parse(&mut self) -> Box<Node> {
        let res = self.expr();
        let ct = self.get_current_token();
        if ct != EOF {
            panic!("Unexpected token at the end of file, got {}", ct)
        }
        return res;
    }
}

fn get_int(t: Token) -> i32 {
    match t {
        IntConst(digit) => digit.parse().unwrap(),
        unknown => panic!("Unexpected token, want integer, got {}", unknown),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let text = "3 + 21 * 1 + - 7 * 2 - (4 + 6)";
        let mut p = Parser::new(text.into());
        let actual = p.parse();

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
}
