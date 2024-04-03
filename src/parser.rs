use crate::lex::Lexer;

pub struct Parser {
    pub lex: Lexer,
}

impl Parser {
    pub fn program(&self) {
        println!("parsing!")
    }
}
