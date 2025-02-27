use crate::token::Token;

pub struct Tokenizer<'a> {
    input: &'a str,
    current: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer { input, current: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.current < self.input.len() {
            let c = self.next_char();

            match c {
                ' ' | '\n' | '\r' | '\t' => continue, // Skip whitespace
                '0'..='9' => tokens.push(self.number()),
                '"' => tokens.push(self.string_literal()),
                '=' => tokens.push(Token::Equals),
                ';' => tokens.push(Token::Semicolon),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                '{' => tokens.push(Token::LBrace),
                '}' => tokens.push(Token::RBrace),
                _ if c.is_alphabetic() => tokens.push(self.identifier_or_keyword(c)),
                _ => {
                    eprintln!("Unexpected character: {}", c);
                    self.current += 1; // Skip the character
                }
            }
        }

        tokens.push(Token::EOF);
        tokens
    }

    fn next_char(&mut self) -> char {
        self.input[self.current..].chars().next().unwrap_or('\0')
    }

    fn number(&mut self) -> Token {
        let start = self.current;
        while self.next_char().is_digit(10) {
            self.current += 1;
        }
        let number: f64 = self.input[start..self.current].parse().unwrap();
        Token::Number(number)
    }

    fn string_literal(&mut self) -> Token {
        self.current += 1; // Skip the opening quote
        let start = self.current;

        while self.next_char() != '"' {
            self.current += 1;
        }
        let string = self.input[start..self.current].to_string();
        self.current += 1; // Skip the closing quote
        Token::StringLiteral(string)
    }

    fn identifier_or_keyword(&mut self, _first_char: char) -> Token { // Prefix with underscore
        let start = self.current - 1; // Include the first character
        while self.next_char().is_alphanumeric() {
            self.current += 1;
        }
        let identifier = &self.input[start..self.current];

        match identifier {
            "print" => Token::Print,
            "if" => Token::If,
            "while" => Token::While,
            "else" => Token::Else,
            _ => Token::Identifier(identifier.to_string()),
        }
    }
}
