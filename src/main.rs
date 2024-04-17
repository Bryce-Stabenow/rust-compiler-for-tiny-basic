mod emit;
mod lex;
mod parser;

use crate::emit::Emitter;
use crate::lex::Lexer;
use crate::parser::Parser;
use std::fs::read_to_string;
use std::io;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Usage: [file_path]");
    }

    // Trim newline from input
    match parse(args[1].trim_end()) {
        Ok(_) => (),
        Err(_) => panic!("Unable to parse file"),
    };
}

fn parse(file_name: &str) -> io::Result<()> {
    let mut file = match read_to_string(file_name) {
        Ok(f) => f,
        Err(_) => panic!("Unable to read file: {}", file_name),
    };

    file += "\n\0"; // Adding newline and EOF for clarity parsing

    // Initialize Lexer, Parser, and Emitter
    let lex = Lexer::new(file);
    let emit = Emitter::new(String::from("output/output.c"));
    let mut parser = Parser::new(lex);

    // Being parsing
    parser.program();
    emit.write_file()?;

    println!("Parsing complete");
    Ok(())
}

// struct Emitter {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handles_hello_world() {
        parse("test_files/hello.teeny");
    }

    #[test]
    fn it_handles_expressions() {
        parse("test_files/expression.teeny");
    }

    #[test]
    fn it_handles_nested_loops() {
        parse("test_files/nested-loop.teeny");
    }

    #[test]
    fn it_handles_loops() {
        parse("test_files/loop.teeny");
    }

    #[test]
    fn it_handles_complex_programs() {
        parse("test_files/complex.teeny");
    }

    #[test]
    #[should_panic]
    fn it_breaks_on_incorrect_syntax() {
        parse("test_files/test.txt");
    }

    #[test]
    #[should_panic]
    fn it_breaks_on_redeclared_labels() {
        parse("test_files/redeclare.teeny");
    }
}
