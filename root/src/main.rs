use clap::{Command, arg};
use std::fs;
use vestra::interpreter::interpret;
use vestra::parser::parse;
use vestra::tokenizer::Tokenizer;

fn main() {
    let matches = Command::new("Vestra")
        .version("0.3.1")
        .author("Author Name <DrCodfish")
        .about("A custom language interpreter")
        .subcommand(
            Command::new("run").about("Runs a Vestra script").arg(
                arg!(<FILE> "The Vestra script file to run")
                    .required(true)
                    .index(1),
            ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        if let Some(filename) = matches.get_one::<String>("FILE") {
            if let Err(e) = run_script(filename) {
                eprintln!("Error: {}", e)
            }
        }
    }
}

fn run_script(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)
        .map_err(|e| format!("Failed to read file '{}': {}", filename, e))?;

    let mut tokenizer = Tokenizer::new();

    // Tokenize the input
    let tokens = tokenizer.tokenize(&contents);

    // Parse the tokens into commands (assuming parse function returns Vec<Command>)
    let commands = parse(tokens).map_err(|e| format!("Failed to parse tokens: {}", e))?;

    // Now interpret the commands
    interpret(commands).map_err(|e| format!("Error during interpretation: {}", e))?;
    
    Ok(())
}
