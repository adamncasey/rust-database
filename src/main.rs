mod database;
mod query;

use std::io::{self, BufRead};

fn main() {
}

fn repl(input: &str) -> String {
    "Not implemented".to_owned()
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        input_string = line.unwrap();
        println!("Input: {}", line.unwrap());

        repl(line)
    }
}

#[test]
fn test_me() {
    
}
