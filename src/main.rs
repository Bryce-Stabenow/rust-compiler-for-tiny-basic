use std::{
    fs::read_to_string,
    io::{self},
};

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

    let mut lex: Lexer = Lexer {
        data: file,
        current_pos: -1,
        current_char: None,
    };

    while let Some(char) = lex.peek() {
        println!("{}", char);
        lex.next_char();
    }
}

struct Lexer {
    data: String,
    current_pos: i64,
    current_char: Option<char>,
}

impl Lexer {
    fn next_char(&mut self) {
        self.current_pos += 1;
        self.current_char = self.data.chars().nth(self.current_pos as usize);
    }

    fn peek(&self) -> Option<char> {
        self.data.chars().nth((self.current_pos + 1) as usize)
    }
}

// struct Parser {}

// struct Emitter {}
