use crate::{database::Database, planner::{column_names, expr_to_string}};
use crate::planner::Plan;

use sqlparser::ast::{SetExpr, Statement, TableFactor};

#[derive(Debug)]
pub struct DbRows {
    current_row: usize,
    rows: Vec<Vec<String>>,
}

impl DbRows {
    pub fn empty() -> DbRows {
        DbRows {
            current_row: 0,
            rows: Vec::new(),
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
    match &plan.command {
        Statement::CreateTable { name, columns, .. } => {
            db.create_table(&name.to_string(), &column_names(columns))?;
            Ok(DbRows::empty())
        }
        Statement::Insert {
            table_name, source, ..
        } => {
            // Access db table
            let table = match db.tables.get_mut(&table_name.to_string()) {
                None => return Err("Table name not found"),
                Some(table) => table,
            };

            // Check type of source
            let values = match &source.body {
                SetExpr::Values(v) => v,
                _ => return Err("Not supported"),
            };

            // Validate tuple lengths in source
            for tuple in &values.0 {
                if table.column_names.len() != tuple.len() {
                    return Err("Incorrect number of values");
                }
            }

            // Insert rows
            for tuple in &values.0 {
                let tuple = tuple.iter().map(expr_to_string).collect();

                table.values.push(tuple);
            }

            Ok(DbRows::empty())
        }
        Statement::Query(q) => {
            // Get the select
            let select = match &q.body {
                SetExpr::Select(select) => select,
                _ => return Err("set type not supported"),
            };

            // Get the first table mentioned
            let table = match select.from.len() {
                1 => &select.from.get(0).unwrap().relation,
                _ => return Err("incorrect number of tables specified"),
            };

            // Get the name of the table
            let table_name = match table {
                TableFactor::Table { name, .. } => name.to_string(),
                _ => return Err("table facor type not supported"),
            };

            // Access the db table of the same name
            let table = match db.tables.get(&table_name) {
                None => return Err("Table name not found"),
                Some(table) => table,
            };

            // Return the rows
            Ok(DbRows {
                current_row: 0,
                rows: table.values.clone(),
            })
        }
        _ => Err("statement type not supported"),
    }
}
