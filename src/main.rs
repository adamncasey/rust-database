mod database;
mod planner;
mod executor;

use crate::database::Database;
use crate::planner::Plan;
use crate::executor::{DbRows};

fn main() {
    println!("Hello, world!");

    let queries = [
        "CREATE TABLE persons (personid, name, nickname, favourite_takeaway)",
        "INSERT INTO persons VALUES ('one', 'Adam Casey', 'Adam', 'Jahangir')",
        "INSERT INTO persons VALUES ('two', 'Karl Sykes', 'Kalleboballefofallemodalle', 'Maisha')",
        "SELECT * FROM persons"
    ];

    let mut db = Database::new();

    for query in &queries {
        let plan: Plan = planner::plan(query);

        println!("Query {:?}. Plan {:?}", query, &plan);

        let result: Result<DbRows, &'static str> = executor::execute(&plan, &mut db);

        println!("Database {:?}", db);

        if let Ok(mut rows) = result {
            println!("Success");
            while let Some(row) = rows.next() {
                println!("Row: {:?}", row);
            }
        }
        else {
            println!("executor failed: {:?}", result);
        }
    }
}
