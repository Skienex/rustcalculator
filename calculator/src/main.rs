mod parser;

use std::io::Write;

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
        let expr = parser::parse(&buf);
        let result = expr.eval();
        println!("[{i}]: {result}");
        i += 1;
    }
}