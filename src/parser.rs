//src/parser.rs

use crate::token::Token;
use crate::parser::Command as ParserCommand; // Rename here

#[derive(Debug, Clone)] // Add Clone trait here
pub enum Command {
    Print(String),
    Set(String, String),
    If(String, Vec<ParserCommand>, Vec<ParserCommand>), // Use ParserCommand here
    While(String, Vec<ParserCommand>), // Use ParserCommand here
}

// This function should convert tokens into Command
pub fn parse(tokens: Vec<Token>) -> Result<Vec<Command>, String> {
    let mut commands = Vec::new();
    
    let mut index = 0;

    while index < tokens.len() {
        match &tokens[index] {
            Token::Print(msg) => {
                commands.push(Command::Print(msg.clone()));
                index += 1;
            }
            Token::Set(var, value) => {
                commands.push(Command::Set(var.clone(), value.clone()));
                index += 1;
            }
            Token::If(cond, then_cmds, else_cmds) => {
                commands.push(Command::If(
                    cond.clone(),
                    then_cmds.clone(),
                    else_cmds.clone(),
                ));
                index += 1;
            }
            Token::While(cond, cmds) => {
                commands.push(Command::While(cond.clone(), cmds.clone()));
                index += 1;
            }
            _ => {
                return Err(format!("Unrecognized token: {:?}", tokens[index]));
            }
        }
    }

    Ok(commands)
}