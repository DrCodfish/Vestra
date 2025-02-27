/* parser.rs - turns raw script into structured commands*/
use crate::value::Value;

#[derive(Debug)]
pub enum Command {
    Print(String),
    Set(String, Value),
    If { variable: String, value: Value, then_branch: Vec<Command>, else_branch: Vec<Command> },
    While { variable: String, value: Value, body: Vec<Command> },
}

pub fn parse(code: &str) -> Vec<Command> {
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
            if parts.len() == 2 {
                commands.push(Command::Set(parts[0].clone(), parse_value(&parts[1])));
            } else {
                eprintln!("Invalid assignment: {}", line);
            }
        } else if !line.is_empty() {
            eprintln!("Unknown command: {}", line);
        }
    }

    commands
}

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
