use std::collections::HashMap;
use std::io::{self, Write};
use std::env;
use std::fs;

#[derive(Debug, Clone)]
enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
enum Command {
    Print(String),
    Set(String, Value),
    If { variable: String, value: Value, then_branch: Vec<Command>, else_branch: Vec<Command> },
    While { variable: String, value: Value, body: Vec<Command> },
}

// Parse the input code into commands
fn parse(code: &str) -> Vec<Command> {
    let mut commands = Vec::new();
    let mut lines = code.lines().peekable();

    while let Some(line) = lines.next() {
        let line = line.trim();

        if line.starts_with("if") {
            let parts: Vec<_> = line.split_whitespace().collect();
            let variable = parts[1].to_string();
            let value = parse_value(parts[3]);

            let (then_branch, else_branch) = parse_branches(&mut lines);
            commands.push(Command::If { variable, value, then_branch, else_branch });
        } else if line.starts_with("while") {
            let parts: Vec<_> = line.split_whitespace().collect();
            let variable = parts[1].to_string();
            let value = parse_value(parts[3]);

            let body = parse_indented_block(&mut lines);
            commands.push(Command::While { variable, value, body });
        } else if line.starts_with("print") {
            commands.push(Command::Print(line[6..].to_string()));
        } else if line.contains("=") {
            let parts: Vec<_> = line.splitn(2, '=').map(|s| s.trim().to_string()).collect();
            commands.push(Command::Set(parts[0].clone(), parse_value(&parts[1])));
        } else {
            panic!("Unknown command: {}", line);
        }
    }

    commands
}

// Parse a value to determine its type (string, number, etc.)
fn parse_value(input: &str) -> Value {
    if let Ok(num) = input.parse::<f64>() {
        Value::Number(num)
    } else if input == "true" {
        Value::Boolean(true)
    } else if input == "false" {
        Value::Boolean(false)
    } else if input == "nil" {
        Value::Nil
    } else {
        Value::String(input.to_string())
    }
}

// Parse the branches of if-else statements
fn parse_branches(lines: &mut std::iter::Peekable<std::str::Lines>) -> (Vec<Command>, Vec<Command>) {
    let mut then_branch = Vec::new();
    let mut else_branch = Vec::new();
    let mut in_else = false;

    while let Some(&next_line) = lines.peek() {
        if next_line.trim().starts_with("else") {
            in_else = true;
            lines.next();
            continue;
        }

        if next_line.starts_with(' ') || next_line.starts_with('\t') {
            let command = parse_line(next_line.trim());
            if in_else {
                else_branch.push(command);
            } else {
                then_branch.push(command);
            }
            lines.next();
        } else {
            break;
        }
    }

    (then_branch, else_branch)
}

// Parse blocks of indented code (used in loops, if branches, etc.)
fn parse_indented_block(lines: &mut std::iter::Peekable<std::str::Lines>) -> Vec<Command> {
    let mut block = Vec::new();

    while let Some(&next_line) = lines.peek() {
        if next_line.starts_with(' ') || next_line.starts_with('\t') {
            block.push(parse_line(next_line.trim()));
            lines.next();
        } else {
            break;
        }
    }

    block
}

// Parse a single line into a command
fn parse_line(line: &str) -> Command {
    if line.starts_with("print") {
        Command::Print(line[6..].to_string())
    } else if line.contains("=") {
        let parts: Vec<_> = line.splitn(2, '=').map(|s| s.trim().to_string()).collect();
        Command::Set(parts[0].clone(), parse_value(&parts[1]))
    } else {
        panic!("Unknown command: {}", line)
    }
}

// Execute the parsed commands
fn interpret(commands: Vec<Command>, context: &mut HashMap<String, Value>) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for command in commands {
        match command {
            Command::Print(text) => writeln!(handle, "{}", text).unwrap(),
            Command::Set(key, value) => { context.insert(key, value); },
            Command::If { variable, value, then_branch, else_branch } => {
                if context.get(&variable) == Some(&value) {
                    interpret(then_branch, context);
                } else {
                    interpret(else_branch, context);
                }
            }
            Command::While { variable, value, body } => {
                while context.get(&variable) == Some(&value) {
                    interpret(body.clone(), context);
                }
            }
        }
    }
}

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a Vestra script file.");
        return;
    }

    let file_path = &args[1];

    // Read the Vestra script from the file
    let code = fs::read_to_string(file_path).expect("Failed to read file");

    // Initialize the context and interpret the parsed commands
    let mut context = HashMap::new();
    let commands = parse(&code);
    interpret(commands, &mut context);
}
