/* interpreter.rs - executes parsed code*/
use crate::parser::Command;
use crate::value::Value;
use std::collections::HashMap;
use std::io::{self, Write};

pub fn interpret(commands: Vec<Command>, context: &mut HashMap<String, Value>) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for command in commands {
        match command {
            Command::Print(text) => writeln!(handle, "{}", text)?,
            Command::Set(key, value) => { context.insert(key, value); },
            Command::If { variable, value, then_branch, else_branch } => {
                if context.get(&variable) == Some(&value) {
                    interpret(then_branch, context)?;
                } else {
                    interpret(else_branch, context)?;
                }
            }
            Command::While { variable, value, body } => {
                while context.get(&variable) == Some(&value) {
                    interpret(body.clone(), context)?;
                }
            }
        }
    }

    Ok(())
}
