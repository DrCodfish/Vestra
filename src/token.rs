/*token.rs*/
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Print,
    If,
    While,
    Else,
    Identifier(String),
    Number(f64),
    StringLiteral(String),
    Equals,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    EOF,
}
