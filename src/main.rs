mod database;
mod query;

#[cfg(test)]
mod test;

use std::io::{self, BufRead};

fn repl(_input: &str) -> String {
    "Not implemented".to_owned()
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input_string = line.unwrap();
        println!("Input: {}", input_string);

        repl(&input_string);
    }
}