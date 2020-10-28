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

    pub fn next(&mut self) -> Option<&Vec<String>> {
        if self.current_row < self.rows.len() {
            // Could now go and grab the row from the DB, but actually we copied it all
            let row_num = self.current_row;
            self.current_row += 1;
            Some(&self.rows[row_num])
            
        } else {
            None
        }
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
        Operation::Insert {
            table_name,
            values
        } => {
            let table = match db.tables.get_mut(table_name) {
                None => return Err("Table name not found"),
                Some(table) => table,
            };

            if table.column_names.len() != values.len() {
                return Err("Incorrect number of values");
            }

            table.values.push(values.to_vec());

            Ok(DbRows::empty())
        },
        Operation::Select {
            table_name
        } => {
            let table = match db.tables.get_mut(table_name) {
                None => return Err("Table name not found"),
                Some(table) => table,
            };

            Ok(DbRows {
                current_row: 0,
                rows: table.values.clone(),
            })
        },
        
        _ => Err("Unknown operation"),
    }
}
