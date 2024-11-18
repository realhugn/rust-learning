use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens
        }
    }

    pub fn parse(&self) -> Result<(), String> {
        let mut stack = vec![];
        for token in &self.tokens {
            match token {
                Token::CurlyOpen => stack.push(token),
                Token::CurlyClose => {
                    if stack.pop() != Some(&Token::CurlyOpen) {
                        return Err("Invalid JSON".to_string());
                    }
                },
                _ => {}
            }
        }
        if stack.is_empty() {
            Ok(())
        } else {
            Err("Invalid JSON".to_string())
        }
    }
}
