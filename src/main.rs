mod scanner;
mod token;
mod token_type;

use std::{
    env::args,
    fs::read_to_string,
    io::{stdin, stdout, Write},
    process::exit,
};

fn run_file(path: String) {
    let contents = read_to_string(path).unwrap_or_default();
    run(contents);
}

fn run_prompt() {
    let mut buf = String::new();

    loop {
        print!("> ");
        stdout().flush().expect("Failed to flush stdout...");
        stdin().read_line(&mut buf).expect("Failed to read line...");
        buf = buf.trim().to_string();

        if buf.is_empty() {
            println!("Exiting...");
            break;
        }

        run(buf.clone());
    }
}

fn run(source: String) {
    println!("{}", source);
}

fn main() {
    let args = args().skip(1).collect::<Vec<String>>();

    match args.len() {
        2.. => {
            eprintln!("Usage: rlox [script]");
            exit(64);
        }
        1 => run_file(args.last().unwrap().to_string()),
        _ => run_prompt(),
    }

    println!("{:#?}", args);
}
