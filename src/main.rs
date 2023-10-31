mod token;

use anyhow::Result;
use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(args.first().expect("file path")),
        _ => {
            println!("Usage: rlox [script]");
            exit(64);
        }
    }
}

fn run_prompt() -> Result<()> {
    let mut stdout = io::stdout();
    print!("> ");
    stdout.flush()?;
    for line in io::stdin().lines() {
        let input = line?;
        if input.is_empty() {
            break;
        }
        run(&input)?;
        print!("> ");
        stdout.flush()?;
    }
    Ok(())
}

fn run_file(path: &str) -> Result<()> {
    let content = fs::read_to_string(path)?;
    run(&content)
}

fn run(source: &str) -> Result<()> {
    match token::scan(source) {
        Ok(_) => {}
        Err(errors) => {
            for (line, _, message) in errors {
                eprintln!("[line {line}] Error: {message}");
            }
        }
    };
    Ok(())
}
