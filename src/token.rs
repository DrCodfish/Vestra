// src/token.rs

use crate::parser::Command;

#[derive(Clone, Debug)]
pub enum Token {
    Print(String),
    Set(String, String),
    If(String, Vec<Command>, Vec<Command>),
    While(String, Vec<Command>),
    Identifier(String),
    Number(f64),
    StringLiteral(String),
    Boolean(bool),
    Nil,
    EOF,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
}
