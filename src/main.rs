/* main.rs - entry point for the Vestra script*/
use vestra::interpreter::interpret;
use vestra::parser::parse;
use std::collections::HashMap;
use std::env;
use std::fs;
use vestra::value::Value;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a Vestra script file.");
        return;
    }

    let file_path = &args[1];
    let code = match fs::read_to_string(file_path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };

    let mut context = HashMap::new();
    let commands = parse(&code);
    if let Err(e) = interpret(commands, &mut context) {
        eprintln!("Error during interpretation: {}", e);
    }
}
