mod error;
mod parser;

use std::{io::Write, panic::catch_unwind};

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();
    let mut i = 1;
    loop {
        buf.clear();
        print!(">>> ");
        stdout.flush().unwrap();
        stdin.read_line(&mut buf).unwrap();
        let expr = catch_unwind(|| parser::parse(&buf));
        if expr.is_err() {
            eprintln!("ERROR: Exception in parser");
            continue;
        }
        let expr = expr.unwrap();
        if let Err(err) = expr {
            eprintln!("ERROR: {err}");
            continue;
        }
        let result = expr.unwrap().eval();
        println!("[{i}]: {result}");
        i += 1;
    }
}
