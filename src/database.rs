#![allow(dead_code)]
use crate::query::{Column, Output, Query, QueryResult};

enum Value {
    FixedString(String),
    I32(i32),
}

struct Row {
    values: Vec<Value>,
}

struct Table {
    // Primary Key is column[0]
    columns: Vec<Column>,
    data: Vec<Row>,
}
impl Table {
    pub fn new(columns: Vec<Column>) -> Table { Table {columns: columns, data: vec![]}}
    pub fn insert(&mut self, _rows: &Vec<String>) {}
    pub fn rows(&self) -> Vec<Vec<String>> {
        vec![]
    }
}

pub struct Database {
    table: Option<Box<Table>>,
}

impl Database {
    pub fn new() -> Database {
        Database { table: None }
    }

    pub fn query(&mut self, query: &Query) -> QueryResult {
        println!("query: {}", serde_json::to_string(&query).unwrap());

        let mut comment = String::new();
        let mut num_rows = 0;

        let rows = match query {
            Query::Select => match &mut self.table {
                Some(table) => {
                    let rows = table.rows();
                    num_rows = rows.len();
                    rows
                }
                None => {
                    comment = "No table".to_owned();
                    vec![]
                }
            },
            Query::Insert { values } => match &mut self.table {
                Some(table) => {
                    table.insert(&values);
                    num_rows = 1;
                    vec![]
                }
                None => {
                    comment = "No table".to_owned();
                    vec![]
                }
            },
            _ => {
                comment = "Not implemented".to_owned();
                vec![]
            }
        };

        let result = Output {
            num_rows: num_rows,
            rows: rows,
            comment: comment,
        };
        println!("result: {}", serde_json::to_string(&result).unwrap());

        Ok(result)
    }

    pub fn json_query(&mut self, query: &str) -> String {
        let query_decoded: Query = serde_json::from_str(&query).unwrap();
        let results_coded = self.query(&query_decoded).unwrap();

        serde_json::to_string(&results_coded).unwrap()
    }
}
