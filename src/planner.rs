#[derive(Debug)]
pub enum Plan {
    CreateTable {
        name: String,
        column_names: Vec<String>,
    },
    Insert {
        table_name: String,
        values: Vec<String>,
    },
    Select {
        table_name: String,
        column_names: Vec<String>,
    },
}

pub fn plan(input: &str) -> Plan {
    Plan::CreateTable {
        name: "test-table".to_string(),
        column_names: vec!["column1".to_string(), "column2".to_string()],
    }
}
