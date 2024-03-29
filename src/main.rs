use std::{char, fs::read_to_string, io};

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

    // let mut lex: Lexer = Lexer {
    //     data: file,
    //     current_pos: -1,
    //     current_char: None,
    // };

    // while let Some(char) = lex.peek() {
    //     println!("{}", char);
    //     lex.next_char();
    // }

    let mut lex2: Lexer = Lexer {
        data: String::from("+- */ >>= = !=\n"),
        current_pos: -1,
        current_char: None,
    };

    lex2.get_token();
    while let Some(token) = lex2.get_token() {
        if token.token_type == TokenType::EOF {
            break;
        }
        println!("{:?}", token.token_type)
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

    fn get_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if let Some(char) = self.current_char {
            let token = match char {
                '+' => Token::new(self.current_char, TokenType::PLUS),
                '-' => Token::new(self.current_char, TokenType::MINUS),
                '/' => Token::new(self.current_char, TokenType::SLASH),
                '*' => Token::new(self.current_char, TokenType::ASTERISK),
                '=' => {
                    if self.peek() == Some('=') {
                        self.next_char();
                        Token::new(None, TokenType::EQEQ)
                    } else {
                        Token::new(self.current_char, TokenType::EQ)
                    }
                }
                '>' => {
                    if self.peek() == Some('=') {
                        self.next_char();
                        Token::new(None, TokenType::GTEQ)
                    } else {
                        Token::new(self.current_char, TokenType::GT)
                    }
                }
                '!' => {
                    if self.peek() == Some('=') {
                        self.next_char();
                        Token::new(None, TokenType::NOTEQ)
                    } else {
                        panic!("Expected !=, got ! Char: {}", self.current_pos);
                    }
                }
                '<' => {
                    if self.peek() == Some('=') {
                        self.next_char();
                        Token::new(None, TokenType::LTEQ)
                    } else {
                        Token::new(self.current_char, TokenType::LT)
                    }
                }
                '\0' => Token::new(None, TokenType::EOF),
                '\n' => Token::new(self.current_char, TokenType::NEWLINE),
                _ => return None,
            };

            self.next_char();
            Some(token)
        } else {
            self.next_char();
            return None;
        }
    }

    fn skip_whitespace(&mut self) {
        if let Some(char) = self.current_char {
            match char {
                ' ' | '\t' | '\r' => self.next_char(),
                _ => return,
            }
        }
    }
}

struct Token {
    token_text: Option<char>,
    token_type: TokenType,
}

impl Token {
    fn new(token_text: Option<char>, token_type: TokenType) -> Self {
        Token {
            token_text,
            token_type,
        }
    }
}

#[derive(PartialEq, Debug)]
enum TokenType {
    EOF,
    NEWLINE,
    NUMBER,
    IDENT,
    STRING,
    //Keywords
    LABEL,
    GOTO,
    PRINT,
    INPUT,
    LET,
    IF,
    THEN,
    ENDIF,
    WHILE,
    REPEAT,
    ENDWHILE,
    //Operators
    EQ,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    EQEQ,
    NOTEQ,
    LT,
    LTEQ,
    GT,
    GTEQ,
}

// struct Parser {}

// struct Emitter {}
