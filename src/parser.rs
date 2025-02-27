/*parser.rs*/
use crate::token::Token;
use crate::value::Value;

#[derive(Debug, Clone)]
pub enum Command {
    Print(String),
    Set(String, Value),
    If { variable: String, value: Value, then_branch: Vec<Command>, else_branch: Vec<Command> },
    While { variable: String, value: Value, body: Vec<Command> },
}

pub fn parse(tokens: Vec<Token>) -> Vec<Command> {
    let mut commands = Vec::new();
    let mut iter = tokens.into_iter().peekable();

    while let Some(token) = iter.next() {
        match token {
            Token::If => {
                // Expect the next token to be an identifier (variable)
                if let Some(Token::Identifier(var_name)) = iter.next() {
                    // Expect the next token to be an Equals token
                    if let Some(Token::Equals) = iter.next() {
                        // Expect the next token to be a value
                        if let Some(value_token) = iter.next() {
                            let value = match value_token {
                                Token::Number(num) => Value::Number(num),
                                Token::StringLiteral(s) => Value::String(s),
                                Token::Identifier(id) => Value::String(id), // Adjust as needed
                                _ => {
                                    eprintln!("Expected a value after '='");
                                    continue;
                                }
                            };

                            // Now we need to parse the then branch
                            let then_branch = parse_block(&mut iter);
                            let else_branch = if let Some(Token::Else) = iter.next() {
                                parse_block(&mut iter)
                            } else {
                                Vec::new()
                            };

                            commands.push(Command::If {
                                variable: var_name,
                                value,
                                then_branch,
                                else_branch,
                            });
                        }
                    }
                }
            }
            Token::While => {
                // Similar logic for while statements
                if let Some(Token::Identifier(var_name)) = iter.next() {
                    if let Some(Token::Equals) = iter.next() {
                        if let Some(value_token) = iter.next() {
                            let value = match value_token {
                                Token::Number(num) => Value::Number(num),
                                Token::StringLiteral(s) => Value::String(s),
                                Token::Identifier(id) => Value::String(id), // Adjust as needed
                                _ => {
                                    eprintln!("Expected a value after '='");
                                    continue;
                                }
                            };

                            let body = parse_block(&mut iter);
                            commands.push(Command::While {
                                variable: var_name,
                                value,
                                body,
                            });
                        }
                    }
                }
            }
            Token::Print => {
                if let Some(Token::StringLiteral(text)) = iter.next() {
                    commands.push(Command::Print(text));
                } else {
                    eprintln!("Expected a string literal after 'print'");
                }
            }
            Token::Identifier(name) => {
                // Handle assignment
                if let Some(Token::Equals) = iter.next() {
                    if let Some(value_token) = iter.next() {
                        let value = match value_token {
                            Token::Number(num) => Value::Number(num),
                            Token::StringLiteral(s) => Value::String(s),
                            Token::Identifier(id) => Value::String(id), // Adjust as needed
                            _ => {
                                eprintln!("Expected a value after '='");
                                continue;
                            }
                        };
                        commands.push(Command::Set(name, value));
                    }
                }
            }
            _ => {
                eprintln!("Unknown token: {:?}", token);
            }
        }
    }

    commands
}

fn parse_block(iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Vec<Command> {
    let mut block = Vec::new();
    while let Some(token) = iter.next() {
        match token {
            Token::Print => {
                if let Some(Token::StringLiteral(text)) = iter.next() {
                    block.push(Command::Print(text));
                } else {
                    eprintln!("Expected a string literal after 'print'");
                }
            }
            Token::Identifier(name) => {
                if let Some(Token::Equals) = iter.next() {
                    if let Some(value_token) = iter.next() {
                        let value = match value_token {
                            Token::Number(num) => Value::Number(num),
                            Token::StringLiteral(s) => Value::String(s),
                            Token::Identifier(id) => Value::String(id), // Adjust as needed
                            _ => {
                                eprintln!("Expected a value after '='");
                                continue;
                            }
                        };
                        block.push(Command::Set(name, value));
                    }
                }
            }
            Token::If => {
                // Handle if statements
                if let Some(Token::Identifier(var_name)) = iter.next() {
                    if let Some(Token::Equals) = iter.next() {
                        if let Some(value_token) = iter.next() {
                            let value = match value_token {
                                Token::Number(num) => Value::Number(num),
                                Token::StringLiteral(s) => Value::String(s),
                                Token::Identifier(id) => Value::String(id), // Adjust as needed
                                _ => {
                                    eprintln!("Expected a value after '='");
                                    continue;
                                }
                            };

                            let then_branch = parse_block(iter);
                            let else_branch = if let Some(Token::Else) = iter.next() {
                                parse_block(iter)
                            } else {
                                Vec::new()
                            };

                            block.push(Command::If {
                                variable: var_name,
                                value,
                                then_branch,
                                else_branch,
                            });
                        }
                    }
                }
            }
            Token::While => {
                // Handle while statements
                if let Some(Token::Identifier(var_name)) = iter.next() {
                    if let Some(Token::Equals) = iter.next() {
                        if let Some(value_token) = iter.next() {
                            let value = match value_token {
                                Token::Number(num) => Value::Number(num),
                                Token::StringLiteral(s) => Value::String(s),
                                Token::Identifier(id) => Value::String(id), // Adjust as needed
                                _ => {
                                    eprintln!("Expected a value after '='");
                                    continue;
                                }
                            };

                            let body = parse_block(iter);
                            block.push(Command::While {
                                variable: var_name,
                                value,
                                body,
                            });
                        }
                    }
                }
            }
            Token::EOF => break, // End of block
            _ => {
                eprintln!("Unknown token: {:?}", token);
            }
        }
    }
    block
}
