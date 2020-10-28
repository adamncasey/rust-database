use crate::database::Database;
use crate::planner::{Operation, Plan};

#[derive(Debug)]
pub struct DbRows {
    current_row: usize,
    rows: Vec<Vec<String>>,
}

impl DbRows {
    pub fn empty() -> DbRows {
        DbRows {
            current_row: 0,
            rows: Vec::new()
        }
    }

    pub fn next(&mut self) -> Option<Vec<String>> {
        None
    }
}

pub fn execute(plan: &Plan, db: &mut Database) -> Result<DbRows, &'static str> {
    match &plan.operation {
        Operation::CreateTable {
            table_name,
            column_names,
        } => {
            db.create_table(&table_name, &column_names)?;
            Ok(DbRows::empty())
        },
        // TODO(jp) insert insert logic
        _ => Err("Unknown operation"),
    }
}
