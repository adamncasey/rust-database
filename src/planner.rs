#[derive(Debug)]
pub struct Plan {
    query: String,
    operation: Operation,
}

#[derive(Debug)]
pub enum Operation {
    CreateTable {
        name: String,
        column_names: Vec<String>
    },
    Unknown,
}

pub fn plan(input: &str) -> Plan {
    Plan {
        query: input.to_string(),
        operation: Operation::Unknown
    }
}