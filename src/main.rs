mod database;
mod executor;
mod planner;

use crate::database::Database;

fn main() {
    let mut database = Database::new();

    let queries = vec![
        "CREATE TABLE persons ('personid', 'name', 'nickname', 'favourite_curry')",
        "INSERT INTO persons VALUES ('one', 'Adam', 'Adam', 'tikka massala')",
        "SELECT * FROM persons"
    ];

    for query in queries {
        let plan = planner::plan(query);

        let status = executor::execute(&plan, &mut database);

        println!("Query: {:?}. Plan {:?}. Status {:?}", query, plan, status);
    }

}
