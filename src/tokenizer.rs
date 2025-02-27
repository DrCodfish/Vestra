// src/tokenizer.rs

use crate::token::Token;
use crate::parser::Command; // Import Command

pub struct Tokenizer {
    // Tokenizer's fields here
}

impl Tokenizer {
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        // Add tokens with the correct constructor arguments
        tokens.push(Token::Print("Hello, world!".to_string())); // Correct string argument
        tokens.push(Token::Set("var_name".to_string(), "value".to_string())); // Correct arguments for Set
        tokens.push(Token::If(
            "true".to_string(),
            vec![Command::Print("True branch".to_string())], // Sample Command for If
            vec![Command::Print("False branch".to_string())], // Sample Command for If
        ));
        tokens.push(Token::While(
            "true".to_string(),
            vec![Command::Print("While loop".to_string())], // Sample Command for While
        ));

        // Correct token variants with their respective arguments
        tokens.push(Token::StringLiteral("A string".to_string())); // Correct StringLiteral token
        tokens.push(Token::Number(42.0)); // Correct Number token
        tokens.push(Token::Boolean(true)); // Correct Boolean token
        tokens.push(Token::Nil); // Correct Nil token

        tokens.push(Token::EOF); // End of file token

        tokens // Return the token vector
    }
}