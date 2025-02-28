use vestra::parser::parse;
use vestra::interpreter::interpret;
use vestra::tokenizer::Tokenizer;

fn main() {
    // Initialize the tokenizer
    let mut tokenizer = Tokenizer {
        // Tokenizer initialization here
    };

    // Tokenize the input
    let tokens = tokenizer.tokenize();

    // Parse the tokens into commands (assuming parse function returns Vec<Command>)
    match parse(tokens) {
        Ok(commands) => {
            // Now interpret the commands
            if let Err(e) = interpret(commands) {
                eprintln!("Error during interpretation: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to parse tokens: {}", e);
        }
    }
}
