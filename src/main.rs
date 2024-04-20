mod emit;
mod lex;
mod parser;

use crate::emit::Emitter;
use crate::lex::Lexer;
use crate::parser::Parser;
use std::fs::read_to_string;
use std::io;
use std::process::abort;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: [file_path] [output_file_name]");
        abort();
    }

    // Trim newline from input
    match parse(args[1].trim_end(), args[2].trim_end()) {
        Ok(_) => (),
        Err(_) => panic!("Unable to output file"),
    };
}

fn parse(file_name: &str, output_file_name: &str) -> io::Result<()> {
    let mut file = match read_to_string(file_name) {
        Ok(f) => f,
        Err(_) => panic!("Unable to read file: {}", file_name),
    };

    // Adding newline and EOF for clarity parsing
    file += "\n\0";

    // Initialize Lexer, Parser
    let lex = Lexer::new(file);
    let emit = Emitter::new(String::from(format!("output/{}.c", output_file_name)));

    // Being parsing
    let mut parser = Parser::new(lex, emit);

    parser.program();
    parser.emit.write_file()?;

    println!("Parsing complete");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handles_hello_world() {
        assert!(parse("test_files/hello.teeny", "hello").is_ok());
    }

    #[test]
    fn it_handles_expressions() {
        assert!(parse("test_files/expression.teeny", "expression").is_ok());
    }

    #[test]
    fn it_handles_nested_loops() {
        assert!(parse("test_files/nested-loop.teeny", "nested-loop").is_ok());
    }

    #[test]
    fn it_handles_loops() {
        assert!(parse("test_files/loop.teeny", "loop").is_ok());
    }

    #[test]
    fn it_handles_complex_programs() {
        assert!(parse("test_files/complex.teeny", "complex").is_ok());
    }

    #[test]
    #[should_panic]
    fn it_breaks_on_incorrect_syntax() {
        assert!(parse("test_files/test.txt", "fail").is_err());
    }

    #[test]
    #[should_panic]
    fn it_breaks_on_redeclared_labels() {
        assert!(parse("test_files/redeclare.teeny", "fail2").is_err());
    }
}
