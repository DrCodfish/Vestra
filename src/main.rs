// src/main.rs

use vestra::parser::parse;
use vestra::interpreter::interpret;
use vestra::tokenizer::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer {
        // Tokenizer initialization here
    };

    // Tokenize the input
    let tokens = tokenizer.tokenize();

    // Parse the tokens into commands (assuming parse function returns Vec<Command>)
    let commands = parse(tokens).expect("Failed to parse tokens");

    // Now interpret the commands
    if let Err(e) = interpret(commands) {
        eprintln!("Error during interpretation: {}", e);
    }
}