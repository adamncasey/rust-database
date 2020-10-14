use crate::tuple::{TupleType, TupleSchema};
use std::collections::HashMap;
use crate::dbtable::{DbTable};

pub struct Database {
    tables: HashMap<String, DbTable>
}

impl Database {
    pub fn new() -> Database {
        Database {
            tables: HashMap::new()
        }
    }

    pub fn create(&mut self, name: &str, schema: TupleSchema) -> Option<DbTable> {
        self.tables.insert(name.to_string(), DbTable::new(schema))
    }

    pub fn get(&self, name: &str) -> Option<&DbTable> {
        self.tables.get(name)
    }
}

#[test]
fn create_db_table() {
    let mut db = Database::new();

    let r = db.create("test1", vec![TupleType::UnsignedInt32, TupleType::VarChar]);

    assert!(r.is_none());

    assert!(db.get("test1").is_some());
}