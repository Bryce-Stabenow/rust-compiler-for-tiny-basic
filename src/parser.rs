use std::{collections::HashSet, process::abort};

use crate::lex::{Lexer, Token, TokenType};

pub struct Parser {
    pub lex: Lexer,
    pub current_token: Option<Token>,
    pub peek_token: Option<Token>,
    pub symbols: HashSet<String>,
    pub declared_labels: HashSet<String>,
    pub gotoed_labels: HashSet<String>,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(lex: Lexer) -> Self {
        let mut parser = Parser {
            lex,
            current_token: None,
            peek_token: None,
            symbols: HashSet::new(),
            declared_labels: HashSet::new(),
            gotoed_labels: HashSet::new(),
        };

        // Initialize current_token and peek_token
        while let None = parser.current_token {
            parser.next_token();
        }

        parser
    }

    pub fn program(&mut self) {
        println!("PROGRAM");

        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }

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
            TokenType::IF => {
                println!("STATEMENT-IF");
                self.next_token();
                self.comparison();

                self.match_token(TokenType::THEN);
                self.nl();

                while self.check_token(TokenType::ENDIF) == false {
                    self.statement();
                }

                self.match_token(TokenType::ENDIF);
            }
            TokenType::WHILE => {
                println!("STATEMENT-WHILE");
                self.next_token();
                self.comparison();

                self.match_token(TokenType::REPEAT);
                self.nl();

                while self.check_token(TokenType::ENDWHILE) == false {
                    self.statement();
                }

                self.match_token(TokenType::ENDWHILE);
            }
            TokenType::LABEL => {
                println!("STATEMENT-LABEL");
                self.next_token();

                let text = self
                    .current_token
                    .as_ref()
                    .unwrap()
                    .token_text
                    .as_ref()
                    .unwrap();

                println!("{}", text);

                if self.declared_labels.contains(text) {
                    println!("Redeclaration of label: {}", text);
                    #[cfg(not(test))]
                    abort();

                    #[cfg(test)] // Panic during testing
                    panic!();
                }

                self.declared_labels.insert(text.clone());
                self.match_token(TokenType::IDENT);
            }
            TokenType::GOTO => {
                println!("STATEMENT-GOTO");
                self.next_token();
                self.match_token(TokenType::IDENT);
            }
            TokenType::LET => {
                println!("STATEMENT-LET");
                self.next_token();
                self.match_token(TokenType::IDENT);
                self.match_token(TokenType::EQ);
                self.expression();
            }
            TokenType::INPUT => {
                println!("STATEMENT-INPUT");
                self.next_token();
                self.match_token(TokenType::IDENT);
            }
            _ => {
                println!(
                    "Unexpected expression at {:?}",
                    self.current_token
                        .as_ref()
                        .unwrap()
                        .token_text
                        .as_ref()
                        .unwrap()
                );
                #[cfg(not(test))]
                abort();

                #[cfg(test)] // Panic here during testing
                panic!();
            }
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

    fn comparison(&mut self) {
        println!("COMPARISON");

        self.expression();

        if self.is_comparison_operator() {
            self.next_token();
            self.expression();
        } else {
            println!(
                "Expected comparison operator at {:?}",
                self.current_token.as_ref().unwrap()
            );
            abort();
        }

        while self.is_comparison_operator() {
            self.next_token();
            self.expression();
        }
    }

    fn is_comparison_operator(&self) -> bool {
        self.check_token(TokenType::GT)
            || self.check_token(TokenType::LT)
            || self.check_token(TokenType::GTEQ)
            || self.check_token(TokenType::LTEQ)
            || self.check_token(TokenType::EQEQ)
            || self.check_token(TokenType::NOTEQ)
    }

    fn expression(&mut self) {
        println!("EXPRESSION");

        self.term();

        while self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
            self.next_token();
            self.term();
        }
    }

    fn term(&mut self) {
        println!("TERM");

        self.urnary();

        while self.check_token(TokenType::ASTERISK) || self.check_token(TokenType::SLASH) {
            self.next_token();
            self.urnary();
        }
    }

    fn urnary(&mut self) {
        println!("UNARY");

        if self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
            self.next_token();
        }

        self.primary();
    }

    fn primary(&mut self) {
        println!(
            "PRIMARY: {:?}",
            self.current_token
                .as_ref()
                .unwrap()
                .token_text
                .as_ref()
                .unwrap()
        );

        if self.check_token(TokenType::IDENT) {
            self.next_token();
        } else if self.check_token(TokenType::NUMBER) {
            self.next_token();
        } else {
            println!(
                "Unexpected primary token: {:?}",
                self.current_token.as_ref().unwrap().token_text
            );
            abort();
        }
    }

    fn check_token(&self, kind: TokenType) -> bool {
        kind == self.current_token.as_ref().unwrap().token_type
    }

    fn check_peek(&self, kind: TokenType) -> bool {
        kind == self.peek_token.as_ref().unwrap().token_type
    }

    fn match_token(&mut self, kind: TokenType) {
        let kind_ref = kind.clone();
        if self.check_token(kind) == false {
            println!(
                "Expected token: {:?}, Got token: {:?}",
                kind_ref,
                self.current_token.as_ref().unwrap().token_type
            );
            abort();
        }

        self.next_token()
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lex.get_token();

        // DEBUGGING SECTION
        // if let Some(_) = self.current_token.as_ref() {
        //     println!("Current: {:?}", self.current_token.as_ref());
        //     println!("Peek: {:?}\n", self.peek_token.as_ref());
        // }
    }
}
