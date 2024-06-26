use std::env::{self, Args};
use std::fs;
use std::io::{self, BufRead};

mod error;
mod scanner;
mod token;

fn main() {
    get_args(env::args());
}

fn get_args(args: Args) {
    let args: Vec<String> = args.collect();

    if args.len() > 1 {
        println!("Usage: rslox [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        run_file(args[0]);
    } else {
        run_prompt();
    }
}

fn run_file(path: String) {
    let lines: Vec<String> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    run(lines);
}

fn run_prompt() {
    let mut buffer = String::new();
    let stdin = io::stdin();

    loop {
        println!("> ");
        let mut line = stdin.lock().read_line(&mut buffer).unwrap();
        run(line);
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens: Vec<Tokens> = scanner.scan_tokens();
}
