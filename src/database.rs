#![allow(dead_code)]
use crate::query::{Column, Output, Query, QueryResult, DataType};
use std::cmp;

#[derive(Clone)]
enum Value {
    FixedString(String),
    I32(i32),
    Null,
}

struct Row {
    values: Vec<Value>,
}
impl Row {
    pub fn to_string_array(&self) -> Vec<String> {
        self.values.iter().map(|v| match v {
            Value::FixedString(s) => s.clone(),
            Value::I32(v) => v.to_string(),
            Value::Null => "NULL".to_owned()
        }).collect::<Vec<String>>().to_vec()
    }
}

struct Table {
    // Primary Key is column[0]
    columns: Vec<Column>,
    data: Vec<Row>,
}
impl Table {
    pub fn new(columns: &Vec<Column>) -> Table {
        Table {columns: columns.to_vec(), data: vec![]}
    }

    pub fn insert(&mut self, values: &Vec<String>) {
        let row_values = values.iter().zip(self.columns.iter()).map(|(val, col)| match col.data_type {
            DataType::FixedString{len} => Value::FixedString(val[0..cmp::min(len, val.len())].to_owned()),
            DataType::I32 => match val.parse::<i32>() {
                Ok(int_val) => Value::I32(int_val),
                _ => Value::Null
            }
        }).collect();
        self.data.push(Row {values: row_values})
    }

    pub fn rows(&self) -> Vec<Vec<String>> {
        let rows = self.data.iter();
        rows.map(|r| r.to_string_array()).collect()
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
            Query::CreateTable { types } => match &mut self.table {
                Some(_) => {
                    comment = "Table alredy exists".to_owned();
                    vec![]
                },
                None => {
                    self.table = Some(Box::new(Table::new(types)));
                    vec![]
                }
            },
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
