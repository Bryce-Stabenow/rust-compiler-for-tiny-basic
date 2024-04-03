mod lex;
mod parser;

use crate::lex::Lexer;
use crate::parser::Parser;
use std::{fs::read_to_string, io};

fn main() {
    println!("Enter path of Tiny-BASIC file to compile:");

    let mut user_file_path = String::new();

    io::stdin()
        .read_line(&mut user_file_path)
        .expect("Error: unable to read input");

    // Trim newline from input
    let mut file = match read_to_string(&user_file_path.trim_end()) {
        Ok(f) => f,
        Err(_) => panic!("Unable to locate file: {}", user_file_path),
    };

    file += "\n"; // Adding newline for clarity parsing end of file

    // Initialize Lexer and Parser
    let mut lex: Lexer = Lexer {
        data: file,
        current_pos: -1,
        current_char: None,
    };

    let mut parser = Parser { lex };

    // Being parsing
    parser.program();

    println!("Parsing complete");

    // lex.get_token();
    // while let Some(token) = lex.get_token() {
    //     if token.token_type == TokenType::EOF {
    //         break;
    //     }

    //     println!("{:?}", token.token_type);

    //     if token.token_type == TokenType::STRING || token.token_type == TokenType::NUMBER {
    //         println!("{}", token.token_text.unwrap())
    //     }
    // }
}

// struct Emitter {}
