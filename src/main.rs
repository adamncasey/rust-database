mod database;
mod dbtable;
mod table;
mod tuple;
mod cell;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::database::Database;
use crate::tuple::TupleSchema;
use crate::tuple::TupleType;

const HISTORY_FILE: &str = ".rust-database-history.txt";

fn parse_schema(tokens: &[&str]) -> Result<TupleSchema, &'static str> {
    let schema: TupleSchema = tokens
        .iter()
        .map(|t| match *t {
            "int" => Some(TupleType::SignedInt32),
            "varchar" => Some(TupleType::VarChar),
            _ => {
                //println!("unrecognised token: expected [int|varchar] {:?} {}", t.kind, t.text);
                None
            }
        })
        .flat_map(|x| x)
        .collect();

    match schema.len() {
        0 => Err("no column types provided"),
        _ => Ok(schema),
    }
}

fn create(table_name: &str, types: &[&str], db: &mut Database) {
    let tuple_schema = parse_schema(types);

    if tuple_schema.is_err() {
        println!(
            "input: create: expected table_name col_type+, got bad schema: {}",
            tuple_schema.unwrap_err()
        );
        return;
    }
    let tuple_schema = tuple_schema.unwrap();

    let existing = db.create(table_name, tuple_schema);
    match existing {
        Some(_) => {
            println!("input: create: overwritten table with same name");
        }
        None => {
            println!(
                "input: create: created table {:?}",
                db.get(table_name).unwrap()
            );
        }
    }
}

fn select(table_name: &str, db: &mut Database) {
    //TODO: implement
}

fn insert(table_name: &str, values: &[&str], db: &mut Database) {
    //TODO: implement
}

fn repl(input: &str, db: &mut Database) {
    let tokens: Vec<&str> = input.split(" ").collect();

    match tokens.as_slice() {
        ["select", table_name, ..] => select(table_name, db),
        ["insert", table_name, ..] => insert(table_name, &tokens[2..], db),
        ["create", table_name, ..] => create(table_name, &tokens[2..], db),
        _ => {
            println!("input: unsupported input: '{:?}'", tokens);
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
            }
            Err(ReadlineError::Interrupted) => {
                println!("rust-database: exiting due to keyboard interrupt");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("rust-database: exiting due to end of input");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(HISTORY_FILE).unwrap();
}
