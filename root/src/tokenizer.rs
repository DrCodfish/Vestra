use crate::parser::Command;
use crate::token::Token;

pub struct Tokenizer {
    // Tokenizer's fields here
}

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer {
            // Initialize fields here
        }
    }

    pub fn tokenize(&mut self, _input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        // Tokenization logic based on `_input` here
        // For now, let's add some hardcoded tokens for testing
        tokens.push(Token::Print("Hello, world!".to_string()));
        tokens.push(Token::Set("var_name".to_string(), "value".to_string()));
        tokens.push(Token::If(
            "true".to_string(),
            vec![Command::Print("True branch".to_string())],
            vec![Command::Print("False branch".to_string())],
        ));
        tokens.push(Token::While(
            "true".to_string(),
            vec![Command::Print("While loop".to_string())],
        ));

        // Example tokens that might be encountered in a real script
        tokens.push(Token::StringLiteral("A string".to_string()));
        tokens.push(Token::Number(42.0));
        tokens.push(Token::Boolean(true));
        tokens.push(Token::Nil);
        tokens.push(Token::EOF);

        tokens
    }
}
