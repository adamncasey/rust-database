use crate::query::{Column, DataType, Query, QueryResult};

enum Value {
    FixedString(String),
    I32(i32)
}

struct Row {
    values: Vec<Value>
}

struct Table {
    // Primary Key is column[0]
    columns: Vec<Column>,
    data: Vec<Row>
}

struct Database {
    table: Table,
}

impl Database {
    fn new() -> Database {
        Database {
            table: Table {
                columns: Vec::new(),
                data: Vec::new()
            }
        }
    }

    fn query(&mut self, query: &Query) -> QueryResult {
        Ok(Output {
            num_rows: 0,
            rows: vec![],
            comment: String::from("Not implemented")
        })
    }
}

#[test]
fn empty_table()
{
    let db = Database::new();

    db.query(Query::CreateTable{
        types: vec![Column {
            name: "personid",
            value: DataType::I32
        },Column {
            name: "age",
            value: DataType::I32
        }]
    });

    let result = db.query(Query::Select);

    assert_eq!(result.unwrap().num_rows, 0);
}


fn insert_select()
{
    let db = Database::new();

    db.query(Query::CreateTable{
        types: vec![Column {
            name: "personid",
            value: DataType::I32
        },Column {
            name: "age",
            value: DataType::I32
        },Column {
            name: "shortname",
            value: DataType::FixedString {
                len: 20
            }
        }]
    });

    db.query(Query::Insert {
        values: vec!["0", "20", "Alice"]
    }).unwrap();

    let result = db.query(Query::Select).unwrap();

    assert_eq!(result.num_rows, 1);

    assert_eq!(result.rows, vec!["0", "20", "Alice"]);
}