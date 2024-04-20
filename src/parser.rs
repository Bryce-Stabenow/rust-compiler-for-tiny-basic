use std::{collections::HashSet, process::abort};

use crate::lex::{Lexer, Token, TokenType};
use crate::Emitter;

pub struct Parser {
    pub lex: Lexer,
    pub emit: Emitter,
    pub current_token: Option<Token>,
    pub peek_token: Option<Token>,
    pub symbols: HashSet<String>,
    pub declared_labels: HashSet<String>,
    pub gotoed_labels: HashSet<String>,
}

impl Parser {
    pub fn new(lex: Lexer, emit: Emitter) -> Self {
        let mut parser = Parser {
            lex,
            emit,
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
        // Initial lines for program
        self.emit.header_line("# include <stdio.h>");
        self.emit.header_line("int main(void){");

        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }

        while self.check_token(TokenType::EOF) == false {
            self.statement();
        }

        // Close file of C
        self.emit.emit_line("return 0;");
        self.emit.emit_line("}");

        for goto in &self.gotoed_labels {
            if !self.declared_labels.contains(goto) {
                println!("Goto called on undeclared label: {}", goto);
                #[cfg(not(test))]
                abort();

                #[cfg(test)] // Panic during testing
                panic!();
            }
        }
    }

    pub fn statement(&mut self) {
        match self.current_token_type() {
            TokenType::PRINT => {
                self.next_token();

                if self.check_token(TokenType::STRING) {
                    let line = format!("printf(\"{}\\n\");", &self.current_token_text());
                    self.emit.emit_line(&line);
                    self.next_token();
                } else {
                    self.emit.emit("printf(\"%.2f\\n\", (float)(");
                    self.expression();
                    self.emit.emit_line("));")
                }
            }
            TokenType::IF => {
                self.next_token();
                self.emit.emit("if(");
                self.comparison();

                self.match_token(TokenType::THEN);
                self.nl();
                self.emit.emit_line("){");

                while self.check_token(TokenType::ENDIF) == false {
                    self.statement();
                }

                self.match_token(TokenType::ENDIF);
                self.emit.emit_line("}");
            }
            TokenType::WHILE => {
                self.next_token();
                self.emit.emit("while(");
                self.comparison();

                self.match_token(TokenType::REPEAT);
                self.nl();
                self.emit.emit_line("){");

                while self.check_token(TokenType::ENDWHILE) == false {
                    self.statement();
                }

                self.match_token(TokenType::ENDWHILE);
                self.emit.emit_line("}");
            }
            TokenType::LABEL => {
                self.next_token();

                let text = &self.current_token_text();

                println!("{}", text);

                if self.declared_labels.contains(text) {
                    println!("Redeclaration of label: {}", text);
                    #[cfg(not(test))]
                    abort();

                    #[cfg(test)] // Panic during testing
                    panic!();
                }

                self.declared_labels.insert(text.clone());
                self.emit.emit(text);
                self.emit.emit_line(":");
                self.match_token(TokenType::IDENT);
            }
            TokenType::GOTO => {
                self.next_token();

                let text = self.current_token_text();

                self.gotoed_labels.insert(text.clone());

                let line = format!("goto {};", text);
                self.emit.emit_line(&line);

                self.match_token(TokenType::IDENT);
            }
            TokenType::LET => {
                self.next_token();

                let text = self.current_token_text();

                if !self.symbols.contains(&text) {
                    self.symbols.insert(text.clone());

                    let decl = format!("float {};", &text);
                    self.emit.header_line(&decl);
                }

                let asgn = format!("{} = ", &text);
                self.emit.emit(&asgn);

                self.match_token(TokenType::IDENT);
                self.match_token(TokenType::EQ);

                self.expression();
                self.emit.emit_line(";");
            }
            TokenType::INPUT => {
                self.next_token();

                let text = self.current_token_text();

                if !self.symbols.contains(&text) {
                    self.symbols.insert(text.clone());

                    let decl = format!("float {};", &text);
                    self.emit.header_line(&decl);
                }

                // Handle our input so we can fallback if a user enters an invalid value for input
                let line = format!("if(0 == scanf(\"%f\", &{})) {{\n{} = 0;", &text, &text);
                self.emit.emit_line(&line);
                self.emit.emit("scanf(\"%");
                self.emit.emit_line("*s\");");
                self.emit.emit_line("}");

                self.match_token(TokenType::IDENT);
            }
            _ => {
                println!("Unexpected expression at {:?}", self.current_token_text());
                #[cfg(not(test))]
                abort();

                #[cfg(test)] // Panic here during testing
                panic!();
            }
        }

        self.nl()
    }

    fn nl(&mut self) {
        self.match_token(TokenType::NEWLINE);

        // Allow for multiple new lines in a row (also handles comments)
        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }
    }

    fn comparison(&mut self) {
        self.expression();

        if self.is_comparison_operator() {
            self.emit.emit(&self.current_token_text());
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
            self.emit.emit(&self.current_token_text());
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
        self.term();

        while self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
            self.emit.emit(&self.current_token_text());
            self.next_token();
            self.term();
        }
    }

    fn term(&mut self) {
        self.urnary();

        while self.check_token(TokenType::ASTERISK) || self.check_token(TokenType::SLASH) {
            self.emit.emit(&self.current_token_text());
            self.next_token();
            self.urnary();
        }
    }

    fn urnary(&mut self) {
        if self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
            self.emit.emit(&self.current_token_text());
            self.next_token();
        }

        self.primary();
    }

    fn primary(&mut self) {
        if self.check_token(TokenType::NUMBER) {
            self.emit.emit(&self.current_token_text());
            self.next_token();
        } else if self.check_token(TokenType::IDENT) {
            let text = self.current_token_text();

            if !self.symbols.contains(&text) {
                println!("Referencing variable before declaration: {}", text);
                #[cfg(not(test))]
                abort();

                #[cfg(test)] // Panic during testing
                panic!();
            }

            self.emit.emit(&self.current_token_text());
            self.next_token();
        } else {
            println!("Unexpected primary token: {:?}", self.current_token_text());
            abort();
        }
    }

    fn check_token(&self, kind: TokenType) -> bool {
        kind == self.current_token_type()
    }

    // fn check_peek(&self, kind: TokenType) -> bool {
    //     kind == self.peek_token.as_ref().unwrap().token_type
    // }

    fn match_token(&mut self, kind: TokenType) {
        let kind_ref = kind.clone();
        if self.check_token(kind) == false {
            println!(
                "Expected token: {:?}, Got token: {:?}",
                kind_ref,
                self.current_token_type()
            );
            abort();
        }

        self.next_token()
    }

    fn current_token_text(&self) -> String {
        self.current_token
            .as_ref()
            .unwrap()
            .token_text
            .as_ref()
            .unwrap()
            .clone()
    }

    fn current_token_type(&self) -> TokenType {
        self.current_token.as_ref().unwrap().token_type.clone()
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
