use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

mod interpreter;
mod lexer;
mod nodes;
mod parser;
mod tokens;
mod transpiler;

macro_rules! repl_print {
    () => {{
        print!(">>> ");
        io::stdout().flush()?;
    }};
}

macro_rules! get_node {
    ($text:expr) => {{
        let mut parser = parser::Parser::new(lexer::Lexer::new($text));
        match parser.parse() {
            Ok(node) => Some(node),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }};
}

fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        match &*args[1] {
            "help" | "-h" | "--help" => {
                println!("usage: calnium [file] [output=a.c]\nusage: calnium (for repl)");
            }
            filename => {
                if filename.trim() != "" {
                    let text = fs::read_to_string(filename)?;
                    if let Some(tree) = get_node!(text.as_str()) {
                        let c_code = transpiler::transpile(tree);
                        let mut output = String::from("a.c");
                        if args.len() == 3 {
                            output = args[2].clone();
                        }
                        fs::write(output, c_code)?;
                    }
                }
            }
        }
    } else {
        let stdin = io::stdin();
        println!("Calnium Repl");
        repl_print!();
        for line in stdin.lock().lines() {
            match line {
                Ok(line) => {
                    if line.trim() != "" {
                        if let Some(node) = get_node!(line.as_str()) {
                            let mut interpreter = interpreter::Interpreter::new();
                            println!("{}", interpreter.interpret(node).value);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
            repl_print!();
        }
    }

    Ok(())
}
