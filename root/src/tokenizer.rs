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

    pub fn tokenize(&mut self, input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                '"' => {
                    let mut str_literal = String::new();
                    i += 1;
                    while i < chars.len() && chars[i] != '"' {
                        str_literal.push(chars[i]);
                        i += 1;
                    }
                    if i < chars.len() && chars[i] == '"' {
                        tokens.push(Token::StringLiteral(str_literal));
                        i += 1;
                    } else {
                        // Handle error: missing closing quote
                        panic!("Unterminated string literal");
                    }
                }
                '0'..='9' => {
                    let mut number = String::new();
                    while i < chars.len() && chars[i].is_numeric() {
                        number.push(chars[i]);
                        i += 1;
                    }
                    tokens.push(Token::Number(number.parse().unwrap()));
                }
                _ => {
                    i += 1;
                }
            }
        }

        tokens.push(Token::EOF);
        tokens
    }
}
