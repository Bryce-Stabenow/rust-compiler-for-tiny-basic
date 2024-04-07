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

        // Initialize current_token and peek_token
        while let None = parser.current_token {
            parser.next_token();
        }

        parser
    }

    pub fn program(&mut self) {
        println!("PROGRAM");

        while self.check_token(TokenType::EOF) == false {
            self.statement();
        }
    }

    pub fn statement(&mut self) {
        match self.current_token.as_ref().unwrap().token_type {
            TokenType::PRINT => {
                println!("STATEMENT-PRINT");
                self.next_token();

                if self.check_token(TokenType::STRING) {
                    self.next_token();
                } else {
                    self.expression();
                }
            }
            _ => panic!("Unexpected expression"),
        }

        self.nl()
    }

    fn nl(&mut self) {
        println!("NEWLINE");
        self.match_token(TokenType::NEWLINE);

        // Allow for multiple new lines in a row (also handles comments)
        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }
    }

    fn expression(&mut self) {}

    fn check_token(&self, kind: TokenType) -> bool {
        kind == self.current_token.as_ref().unwrap().token_type
    }

    fn check_peek(&self, kind: TokenType) -> bool {
        kind == self.peek_token.as_ref().unwrap().token_type
    }

    fn match_token(&mut self, kind: TokenType) {
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

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lex.get_token();
    }
}
