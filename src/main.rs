fn main() -> anyhow::Result<()> {
    use std::io::Write as _;

    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        1 => {
            let mut stdout = std::io::stdout();
            print!("> ");
            stdout.flush()?;
            for line in std::io::stdin().lines() {
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
        2 => {
            let path = args.first().unwrap();
            let content = std::fs::read_to_string(path)?;
            run(&content)
        }
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        }
    }
}

fn run(source: &str) -> anyhow::Result<()> {
    match rlox::token::scan(source) {
        Ok(_) => {}
        Err(errors) => {
            for error in errors {
                eprintln!("{error}");
            }
        }
    };
    Ok(())
}
