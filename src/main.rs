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

    let mut lex: Lexer = Lexer {
        data: file,
        current_pos: -1,
        current_char: None,
    };

    lex.get_token();
    while let Some(token) = lex.get_token() {
        if token.token_type == TokenType::EOF {
            break;
        }

        println!("{:?}", token.token_type);

        if token.token_type == TokenType::STRING || token.token_type == TokenType::NUMBER {
            println!("{}", token.token_text.unwrap())
        }
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
        self.skip_comment();

        if let Some(char) = self.current_char {
            let token: Token;

            if char.is_digit(10) {
                token = Token::new(Some(self.get_number()), TokenType::NUMBER);
            } else if char.is_alphabetic() {
                token = self.get_keyword_token();
            } else {
                token = match char {
                    '+' => Token::new(Some(self.current_char?.to_string()), TokenType::PLUS), // Could unwrap current_char, but this is easier
                    '-' => Token::new(Some(self.current_char?.to_string()), TokenType::MINUS),
                    '/' => Token::new(Some(self.current_char?.to_string()), TokenType::SLASH),
                    '*' => Token::new(Some(self.current_char?.to_string()), TokenType::ASTERISK),
                    '=' => {
                        if self.peek() == Some('=') {
                            self.next_char();
                            Token::new(None, TokenType::EQEQ)
                        } else {
                            Token::new(Some(self.current_char?.to_string()), TokenType::EQ)
                        }
                    }
                    '>' => {
                        if self.peek() == Some('=') {
                            self.next_char();
                            Token::new(Some(">=".to_string()), TokenType::GTEQ)
                        } else {
                            Token::new(Some(self.current_char?.to_string()), TokenType::GT)
                        }
                    }
                    '!' => {
                        if self.peek() == Some('=') {
                            self.next_char();
                            Token::new(Some("!=".to_string()), TokenType::NOTEQ)
                        } else {
                            panic!("Expected !=, got ! Char: {}", self.current_pos);
                        }
                    }
                    '<' => {
                        if self.peek() == Some('=') {
                            self.next_char();
                            Token::new(Some("<=".to_string()), TokenType::LTEQ)
                        } else {
                            Token::new(Some(self.current_char?.to_string()), TokenType::LT)
                        }
                    }
                    '"' => Token::new(Some(self.get_string()), TokenType::STRING),
                    '\0' => Token::new(None, TokenType::EOF),
                    '\n' => Token::new(Some(self.current_char?.to_string()), TokenType::NEWLINE),
                    _ => return None,
                };
            }

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

    fn skip_comment(&mut self) {
        let char = self.current_char.unwrap_or('\0');

        if char == '#' {
            while let Some(char) = self.current_char {
                match char {
                    '\n' => break,
                    _ => self.next_char(),
                }
            }
        }
    }

    fn get_string(&mut self) -> String {
        self.next_char(); // Move cursor to first line of string instead of " char
        let mut string_val = String::new();

        while let Some(char) = self.current_char {
            match char {
                '\n' | '\r' | '%' | '\t' | '\\' => {
                    panic!("Unexpected character in string: {}", char); // Don't allow escape characters, newlines, tabs, or %
                }
                '"' => break,
                _ => {
                    string_val.push(char);
                    self.next_char();
                }
            }
        }

        string_val
    }

    fn get_number(&mut self) -> String {
        let mut num_val = String::new();

        while let Some(char) = self.current_char {
            match char.is_digit(10) || char == '.' {
                true => {
                    num_val.push(char);
                    self.next_char();
                }
                false => break,
            }
        }

        num_val
    }

    fn get_keyword_token(&mut self) -> Token {
        let mut word = String::new();

        while let Some(char) = self.peek() {
            word.push(self.current_char.expect("ERROR: Unable to parse keyword"));

            match char.is_alphabetic() {
                true => {
                    self.next_char();
                }
                false => {
                    break;
                }
            }
        }

        TokenType::from_string(word)
    }
}

struct Token {
    token_text: Option<String>,
    token_type: TokenType,
}

impl Token {
    fn new(token_text: Option<String>, token_type: TokenType) -> Self {
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

impl TokenType {
    fn from_string(input: String) -> Token {
        let token_type = match &input.to_lowercase()[..] {
            "label" => TokenType::LABEL,
            "goto" => TokenType::GOTO,
            "print" => TokenType::PRINT,
            "input" => TokenType::INPUT,
            "let" => TokenType::LET,
            "if" => TokenType::IF,
            "then" => TokenType::THEN,
            "endif" => TokenType::ENDIF,
            "while" => TokenType::WHILE,
            "repeat" => TokenType::REPEAT,
            "endwhile" => TokenType::ENDWHILE,
            _ => TokenType::IDENT,
        };

        Token::new(Some(input), token_type)
    }
}

// struct Parser {}

// struct Emitter {}
