use clap::{Command, arg};
use std::fs;
use vestra::interpreter::interpret;
use vestra::parser::parse;
use vestra::tokenizer::Tokenizer;

fn main() {
    let matches = Command::new("Vestra")
        .version("0.3.0")
        .author("Author Name <author@example.com>")
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
            run_script(filename);
        }
    }
}

fn run_script(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut tokenizer = Tokenizer::new();

    // Tokenize the input
    let tokens = tokenizer.tokenize(&contents);

    // Parse the tokens into commands (assuming parse function returns Vec<Command>)
    let commands = parse(tokens).expect("Failed to parse tokens");

    // Now interpret the commands
    if let Err(e) = interpret(commands) {
        eprintln!("Error during interpretation: {}", e);
    }
}
