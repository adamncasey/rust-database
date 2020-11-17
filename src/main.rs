mod database;
mod executor;
mod planner;

use crate::database::Database;
use crate::executor::DbRows;
use crate::planner::Plan;

fn main() {
    println!("Hello, world!");

    let queries = [
        "CREATE TABLE persons (personid varchar, name varchar, nickname varchar, favourite_takeaway varchar)",
        "INSERT INTO persons VALUES ('one', 'Adam-Casey', 'Adam', 'Jahangir')",
        "INSERT INTO persons VALUES ('two', 'Karl-Sykes', 'Kalleboballefofallemodalle', 'Maisha')",
        "SELECT * FROM persons",
    ];

    let mut db = Database::new();

    for query in &queries {
        let plan = planner::plan(query);

        if plan.is_err() {
            println!("Plan error {:?}", plan.unwrap_err());
            continue;
        }

        let plan = plan.unwrap();

        println!("Query {:#?}", query);
        println!("Plan {:#?}", &plan);

        let result: Result<DbRows, &'static str> = executor::execute(&plan, &mut db);

        println!("Database {:#?}", db);

        if let Ok(mut rows) = result {
            println!("Success");
            while let Some(row) = rows.next() {
                println!("Row: {:?}", row);
            }
        } else {
            println!("executor failed: {:#?}", result);
        }
    }
}
