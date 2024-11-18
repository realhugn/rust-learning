pub struct Lexer {
    input: String
}

#[derive(Debug, PartialEq)]
pub enum Token {
    CurlyOpen,
    CurlyClose,
    Unknown(char)
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input
        }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        self.input.chars().map(|c| match c {
            '{' => Token::CurlyOpen,
            '}' => Token::CurlyClose,
            _ => Token::Unknown(c)
        }).collect()
    }
}

