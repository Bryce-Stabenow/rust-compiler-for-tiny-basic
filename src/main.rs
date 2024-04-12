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

    parse(args[1].trim_end());
}

fn parse(file_name: &str) {
    // Trim newline from input
    let mut file = match read_to_string(file_name) {
        Ok(f) => f,
        Err(_) => panic!("Unable to read file: {}", file_name),
    };

    file += "\n\0"; // Adding newline and EOF for clarity parsing

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handles_all_test_files() {
        let test_files: Vec<&str> = vec![
            "expression.teeny",
            "hello.teeny",
            "loop.teeny",
            "nested-loop.teeny",
        ];

        for file in test_files {
            parse(file);
        }
    }

    #[test]
    #[should_panic]
    fn it_breaks_on_incorrect_tests() {
        parse("test.txt");
    }
}
