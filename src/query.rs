pub struct Output {
    num_rows: usize,
    rows: Vec<Vec<String>>,
    comment: String
}

pub type QueryResult = std::result::Result<Output, String>;

pub enum Query {
    Select,
    Insert {
        values: Vec<String>
    },
    CreateTable{
        types: Vec<Column>
    }
}

pub enum DataType {
    FixedString {
        len: usize
    },
    I32 // 32bit?
}

pub struct Column {
    name: String,
    data_type: DataType,
}
