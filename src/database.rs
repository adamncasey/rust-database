use crate::tuple::{TupleType};
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

    pub fn create(&mut self, name: &str) -> Option<DbTable> {
        self.tables.insert(name.to_string(), DbTable::new(vec![TupleType::UnsignedInt32, TupleType::VarChar]))
    }
}

#[test]
fn create_db_table() {
    let mut db = Database::new();

    let r = db.create("test1");

    assert!(r.is_none());
}