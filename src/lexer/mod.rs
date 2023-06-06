pub mod token;
pub use token::Token;
use token::Token::*;

const RADIX: u32 = 10;

#[derive(Debug)]
pub struct Lexer {
    text: Vec<char>,
    len: usize,
    pos: usize,
    cur_ch: Option<char>,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        let chars: Vec<char> = text.chars().collect();
        let len = chars.len();
        let cur_ch = chars[0];
        Lexer {
            text: chars,
            len: len,
            pos: 0,
            cur_ch: Some(cur_ch),
        }
    }
    fn advance(&mut self) {
        self.pos += 1;
        if self.pos > self.len - 1 {
            self.cur_ch = None;
        } else {
            self.cur_ch = Some(self.text[self.pos])
        }
    }
    fn skip_whitespace(&mut self) {
        while self.cur_ch != None && self.cur_ch.unwrap().is_whitespace() {
            self.advance()
        }
    }
    fn number(&mut self) -> Token {
        let mut digits = String::new();
        while self.cur_ch != None && self.cur_ch.unwrap().is_digit(RADIX) {
            digits.push(self.cur_ch.unwrap());
            self.advance();

            if Some('.') == self.cur_ch {
                digits.push('.');
                self.advance();

                while self.cur_ch != None && self.cur_ch.unwrap().is_digit(RADIX) {
                    digits.push(self.cur_ch.unwrap());
                    self.advance();
                }
                return RealConst(digits);
            }
        }
        IntConst(digits)
    }
    fn id(&mut self) -> Token {
        let mut id = String::new();
        while self.cur_ch != None && self.cur_ch.unwrap().is_alphanumeric() {
            id.push(self.cur_ch.unwrap());
            self.advance();
        }
        ID(id)
    }
    pub fn get_next_token(&mut self) -> Token {
        while self.cur_ch != None {
            return match self.cur_ch.unwrap() {
                char if char.is_whitespace() => {
                    self.skip_whitespace();
                    continue;
                }
                char if char.is_digit(RADIX) => self.number(),
                char if char.is_alphabetic() => self.id(),
                '+' => {
                    self.advance();
                    Plus
                }
                '-' => {
                    self.advance();
                    Minus
                }
                '*' => {
                    self.advance();
                    Multi
                }
                '/' => {
                    self.advance();
                    Div
                }
                '(' => {
                    self.advance();
                    LParan
                }
                ')' => {
                    self.advance();
                    RParan
                }
                char if char.is_alphabetic() => self.id(),
                unknown => panic!("Unknown token found: {}", unknown),
            };
        }
        EOF
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_all_tokens() {
        let text = " 311 eee 3.33 ()+-*/".to_string();
        let mut l = Lexer::new(text);
        assert_eq!(l.get_next_token(), IntConst("311".into()));
        assert_eq!(l.get_next_token(), ID("eee".into()));
        assert_eq!(l.get_next_token(), RealConst("3.33".into()));
        assert_eq!(l.get_next_token(), LParan);
        assert_eq!(l.get_next_token(), RParan);
        assert_eq!(l.get_next_token(), Plus);
        assert_eq!(l.get_next_token(), Minus);
        assert_eq!(l.get_next_token(), Multi);
        assert_eq!(l.get_next_token(), Div);
        assert_eq!(l.get_next_token(), EOF);
    }
}
