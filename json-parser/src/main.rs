mod lexer;
mod parser;

struct App {
    args: Vec<String>,
}

impl App {
    fn new(args: Vec<String>) -> Self {
        App {
            args,
        }
    }

    fn run(&mut self) {
        let input = self.args.join(" ");
        let tokens = lexer::Lexer::new(input).tokenize();
        let parser = parser::Parser::new(tokens);
        match parser.parse() {
            Ok(_) => std::process::exit(0),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut app = App::new(args);
    app.run();
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn step1_valid() {
        let input = fs::read_to_string("tests/step1/valid.json").unwrap();
        let tokens = lexer::Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![lexer::Token::CurlyOpen, lexer::Token::CurlyClose]);
    }

    #[test]
    fn step1_invalid() {
        let input = fs::read_to_string("tests/step1/invalid.json").unwrap();
        let tokens = lexer::Lexer::new(input).tokenize();
        assert_eq!(tokens, vec![]);
    }
}