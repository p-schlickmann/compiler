mod table;
mod lexer;
use lexer::lex;
mod parser;
use parser::parse;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        return;
    }

    let input_file = &args[1];

    match lex(input_file) {
        Ok(tokens) => {
            println!("\n✅ Lexed successfully!");
            println!("Tokens: {:?}\n", tokens);
            // Optionally, you can pass tokens to the parser if needed
            parse(&tokens.join(" "));
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}