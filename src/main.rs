mod tuple;
mod dbtable;
mod database;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use sqlite3_tokenizer::Tokenizer;
use sqlite3_tokenizer::Token;
use sqlite3_tokenizer::TokenKind;

use crate::database::Database;

const HISTORY_FILE: &str = ".rust-database-history.txt";

fn is_whitespace(token: &Token) -> bool {
    match token.kind {
        TokenKind::Space => true,
        _ => false
    }
}

fn repl(input: &str, db: &mut Database) {
    let mut tokens = Tokenizer::new(input).filter(|t| !is_whitespace(&t));

    let token = tokens.next();

    if token.is_none() {
        return;
    }

    let token = token.unwrap();

    match token.kind {
        TokenKind::Select => {
            println!("input: not yet implemented select");
        },
        TokenKind::Insert => {
            println!("input: not yet implemented insert");
        },
        TokenKind::Create => {
            let name = tokens.next();
            if name.is_none() {
                println!("input: create: expected table name, got nothing");
                return;
            }
            let name = name.unwrap();
            let existing = db.create(name.text);
            match existing {
                Some(_) => { println!("input: create: overwritten table with same name");},
                None => { println!("input: create: created table"); }
            }
        },
        _ => {
            println!("input: unsupported input: kind {:?} '{}'", token.kind, token.text);
        }
    }
}

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }

    let mut db = Database::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                repl(line.as_str(), &mut db);
            },
            Err(ReadlineError::Interrupted) => {
                println!("rust-database: exiting due to keyboard interrupt");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("rust-database: exiting due to end of input");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history(HISTORY_FILE).unwrap();
}