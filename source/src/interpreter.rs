use crate::parser::Command;
use std::io;

/// Interprets a vector of commands and executes them.
/// 
/// # Arguments
/// 
/// * `commands` - A vector of commands to be interpreted.
/// 
/// # Returns
/// 
/// An `io::Result` indicating success or failure.
pub fn interpret(commands: Vec<Command>) -> io::Result<()> {
    for command in commands {
        match command {
            Command::Print(msg) => {
                println!("{}", msg);
            }
            Command::Set(var, value) => {
                // Handle the Set command
                println!("Setting {} = {}", var, value);
            }
            Command::If(cond, then_cmds, else_cmds) => {
                // Handle the If command
                println!("If condition: {}", cond);
                interpret(then_cmds)?; // Interpret then commands
                interpret(else_cmds)?; // Interpret else commands
            }
            Command::While(cond, cmds) => {
                // Handle the While command
                println!("While condition: {}", cond);
                interpret(cmds)?; // Interpret while loop commands
            }
        }
    }
    Ok(())
}
