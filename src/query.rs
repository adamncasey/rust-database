use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Output {
    pub num_rows: usize,
    pub rows: Vec<Vec<String>>,
    pub comment: String
}

pub type QueryResult = std::result::Result<Output, String>;

#[derive(Debug, Serialize, Deserialize)]
pub enum Query {
    Select,
    Insert {
        values: Vec<String>
    },
    CreateTable{
        types: Vec<Column>
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataType {
    FixedString {
        len: usize
    },
    I32 // 32bit?
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
}
