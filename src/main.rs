mod lex;
mod parser;

use crate::lex::Lexer;
use crate::parser::Parser;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        panic!("Usage: [file_path]");
    }

    // Trim newline from input
    let mut file = match read_to_string(&args[1].trim_end()) {
        Ok(f) => f,
        Err(_) => panic!("Unable to read file: {}", args[1]),
    };

    file += "\n\0"; // Adding newline for clarity parsing end of file

    // Initialize Lexer and Parser
    let lex: Lexer = Lexer {
        data: file,
        current_pos: -1,
        current_char: None,
    };

    let mut parser = Parser::new(lex);

    // Being parsing
    parser.program();

    println!("Parsing complete");
}

// struct Emitter {}
