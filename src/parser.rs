use crate::lex::Lexer;

pub struct Parser {
    pub lex: Lexer,
}

impl Parser {
    pub fn program(&self) {
        println!("parsing!")
    }

    pub fn check_token(&self) {}

    pub fn check_peek(&self) {}

    pub fn match_token(&self) {}

    pub fn next_token(&self) {}
}
