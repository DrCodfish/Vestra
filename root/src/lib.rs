// src/lib.rs

pub mod parser;
pub mod interpreter;
pub mod tokenizer;
pub mod token;
pub mod value;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_command() {
        let tokens = vec![Token::Print("Hello, world!".to_string())];
        let commands = parse(tokens).expect("Failed to parse tokens");
        assert_eq!(commands.len(), 1);
        if let Command::Print(msg) = &commands[0] {
            assert_eq!(msg, "Hello, world!");
        } else {
            panic!("Expected a Print command");
        }
    }

    #[test]
    fn test_set_command() {
        let tokens = vec![Token::Set("var_name".to_string(), "value".to_string())];
        let commands = parse(tokens).expect("Failed to parse tokens");
        assert_eq!(commands.len(), 1);
        if let Command::Set(var, value) = &commands[0] {
            assert_eq!(var, "var_name");
            assert_eq!(value, "value");
        } else {
            panic!("Expected a Set command");
        }
    }
}
