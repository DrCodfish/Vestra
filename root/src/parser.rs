use crate::parser::Command as ParserCommand;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Command {
    Print(String),
    Set(String, String),
    If(String, Vec<ParserCommand>, Vec<ParserCommand>),
    While(String, Vec<ParserCommand>),
}

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
            Token::StringLiteral(_) | Token::Number(_) | Token::Boolean(_) | Token::Nil | Token::EOF => {
                // Handle or ignore these tokens if they are not required for commands
                index += 1;
            }
            _ => {
                return Err(format!(
                    "Unrecognized token at index {}: {:?}",
                    index, tokens[index]
                ));
            }
        }
    }

    Ok(commands)
}
