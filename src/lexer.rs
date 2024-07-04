use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn lex(file_path: &str) -> Result<Vec<String>, String> {
    let path = Path::new(file_path);
    let file = File::open(&path).map_err(|e| format!("Error opening file: {}", e))?;
    let reader = io::BufReader::new(file);

    let mut tokens = Vec::new();
    let mut line_num = 1;
    let mut col_num = 1;

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Error reading line: {}", e))?;
        let mut chars = line.chars().peekable();

        while let Some(&c) = chars.peek() {
            col_num += 1;
            match c {
                ' ' | '\t' | '\n' | '\r' => {
                    chars.next();
                }
                '(' => {
                    tokens.push("(".to_string());
                    chars.next();
                }
                ')' => {
                    tokens.push(")".to_string());
                    chars.next();
                }
                '{' => {
                    tokens.push("{".to_string());
                    chars.next();
                }
                '}' => {
                    tokens.push("}".to_string());
                    chars.next();
                }
                ';' => {
                    tokens.push(";".to_string());
                    chars.next();
                }
                ',' => {
                    tokens.push(",".to_string());
                    chars.next();
                }
                '=' => {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push("==".to_string());
                    } else {
                        tokens.push("=".to_string());
                    }
                }
                '<' => {
                    tokens.push("<".to_string());
                    chars.next();
                }
                '>' => {
                    tokens.push(">".to_string());
                    chars.next();
                }
                '+' => {
                    tokens.push("+".to_string());
                    chars.next();
                }
                '-' => {
                    tokens.push("-".to_string());
                    chars.next();
                }
                '*' => {
                    tokens.push("*".to_string());
                    chars.next();
                }
                '0'..='9' => {
                    while let Some(&c) = chars.peek() {
                        if c.is_numeric() {
                            chars.next();
                            col_num += 1;
                        } else {
                            break;
                        }
                    }
                    tokens.push("num".to_string());
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut ident = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            ident.push(c);
                            chars.next();
                            col_num += 1;
                        } else {
                            break;
                        }
                    }
                    match ident.as_str() {
                        "if" | "else" | "print" | "return" | "int" | "def" => tokens.push(ident.to_string()),
                        _ => tokens.push("id".to_string()), // Treat as an identifier and classify as "id"
                    }
                }
                _ => {
                    return Err(format!("Lexical error at line {}, column {}", line_num, col_num));
                }
            }
        }
        line_num += 1;
        col_num = 1;
    }

    tokens.push("$".to_string()); // EOF marker
    Ok(tokens)
}