use std::env;
use std::fs;
use std::io;
mod error;
mod scanner;
mod token;

fn run(contents: &str) {
    let mut scanner = scanner::Scanner::new(contents);

    scanner.scan_tokens();

    for token in scanner.get_tokens() {
        println!("{}", token.to_string());
    }
}

fn run_file(file_path: &str) {
    let contents = fs::read_to_string(file_path)
        .expect("Unable to read file"); 
    
    run(contents.as_str());
}

fn get_expression() -> String {
    let mut expression = String::new();

    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read expr");

    expression
        .trim()
        .to_string()
}

fn repl() {
    loop {
        println!(":> ");

        let expression = get_expression();

        if expression.as_str() == ":q" {
            break;
        }

        run(expression.as_str());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        repl();

        return;
    }

    let file_path = &args[2];

    run_file(file_path);
}
