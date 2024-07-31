use std::env::{self, Args};
use std::fs;
use std::io::{self, Write};
use std::process;

mod lox_error;
mod scanner;
mod token;
mod literal;

fn main() -> eyre::Result<()> {
    let args = env::args();
    parse_args(args)?;
    Ok(())
}

fn parse_args(args: Args) -> eyre::Result<()> {
    let iter_args: Vec<String> = args.collect();
    match iter_args.len() {
        1 => run_prompt(),
        2 => run_file(&iter_args[1]),
        _ => {
            println!("Usage: rslox [script]");
            process::exit(64);
        }
    }
}

fn run_prompt() -> eyre::Result<()> {
    let mut input = String::new();
    loop {
        println!(">");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input)?;
        if input.trim().len() < 1 {
            break;
        }
        run(&input);
        input.clear();
    }
    Ok(())
}

fn run_file(path: &str) -> eyre::Result<()> {
    let lines = fs::read_to_string(path)?
        .lines()
        .map(String::from)
        .collect();
    run(&lines);
    Ok(())
}

fn run(source: &String) {}
