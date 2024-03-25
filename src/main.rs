use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    println!("Enter path of Tiny-BASIC file to compile:");

    let mut user_file_path = String::new();

    io::stdin()
        .read_line(&mut user_file_path)
        .expect("Error: unable to read input");

    let mut file = match File::open(user_file_path) {
        Ok(f) => f,
        Err(_) => panic!("Unable to locate file"),
    };

    let mut file_string = String::new();

    // Fine to unwrap here, this would fail only if we can't allocate enough memory to load the file in heap
    file.read_to_string(&mut file_string).unwrap();

    let lex: Lexer = Lexer {
        data: file_string + "\n",
        current_pos: -1,
        current_char: None,
    };
}

struct Lexer {
    data: String,
    current_pos: i64,
    current_char: Option<char>,
}

impl Lexer {}

// struct Parser {}

// struct Emitter {}
