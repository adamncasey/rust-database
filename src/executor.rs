use crate::planner::Plan;
use crate::database::Database;

#[derive(Debug)]
pub struct DbRows {
    current_row: usize,
    rows: Vec<Vec<String>>
}

impl DbRows {
    pub fn next(&mut self) -> Option<Vec<String>> {
        None
    }
}


pub fn execute(plan: &Plan, db: &mut Database) -> Result<DbRows, &'static str> {
    Err("Not implemented")
}