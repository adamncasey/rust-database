use std::collections::HashMap;

#[derive(Debug)]
pub struct Table {
    pub column_names: Vec<String>,
    pub values: Vec<Vec<String>>
}

#[derive(Debug)]
pub struct Database {
    pub tables: HashMap<String, Table>
}

impl Table {
    pub fn new(column_names: &[String]) -> Table {
        Table {
            column_names: column_names.to_vec(),
            values: Vec::new()
        }
    }
}

impl Database {
    pub fn new() -> Database {
        Database{
            tables: HashMap::new()
        }
    }

    pub fn create_table(&mut self, table_name: &str, column_names: &[String]) -> Result<(), &'static str>{
        self.tables.insert(table_name.to_string(), Table::new(column_names));

        Ok(())
    }
}