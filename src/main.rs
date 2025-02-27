/*main.rs*/
use vestra::interpreter::interpret;
use vestra::parser::parse;
use vestra::tokenizer::Tokenizer; // Import the Tokenizer
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a Vestra script file.");
        return;
    }

    let file_path = &args[1];

    // Check for the .vs extension
    if !file_path.ends_with(".vs") {
        eprintln!("Error: The file must have a .vs extension.");
        return;
    }

    let code = match fs::read_to_string(file_path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };

    let mut tokenizer = Tokenizer::new(&code);
    let tokens = tokenizer.tokenize(); // Tokenize the code
    let commands = parse(tokens); // Parse the tokens

    let mut context = HashMap::new();
    if let Err(e) = interpret(commands, &mut context) {
        eprintln!("Error during interpretation: {}", e);
    }
}
