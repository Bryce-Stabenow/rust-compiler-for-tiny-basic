use crate::lex::{Lexer, Token, TokenType};

pub struct Parser {
    pub lex: Lexer,
    pub current_token: Option<Token>,
    pub peek_token: Option<Token>,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(lex: Lexer) -> Self {
        let mut parser = Parser {
            lex,
            current_token: None,
            peek_token: None,
        };

        // Initialize the current_token and peek_token
        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn program(&self) {
        println!("parsing!")
    }

    pub fn check_token(&self, kind: TokenType) -> bool {
        kind == self.current_token.as_ref().unwrap().token_type
    }

    pub fn check_peek(&self, kind: TokenType) -> bool {
        kind == self.peek_token.as_ref().unwrap().token_type
    }

    pub fn match_token(&mut self, kind: TokenType) {
        let kind_ref = kind.clone();
        if self.check_token(kind) == false {
            panic!(
                "Expected token: {:?}, Got token: {:?}",
                kind_ref,
                self.current_token.as_ref().unwrap().token_type
            );
        }

        self.next_token()
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lex.get_token();
    }
}
