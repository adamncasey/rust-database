use crate::table::{RowId, Table, TableRange};
use crate::tuple::{Binerable, Tuple, TupleSchema, TupleType, TupleVariant};
use std::fmt;

pub struct DbTable {
    table: Table,
    schema: TupleSchema,
    next_rowid: RowId,
}

struct DbTableRange<'a> {
    range: TableRange<'a>,
}

impl Iterator for DbTableRange<'_> {
    type Item = (RowId, Tuple);
    fn next(&mut self) -> Option<Self::Item> {
        match self.range.next() {
            Some((rowid, row_data)) => {
                let decode_result = Tuple::deserialize(row_data);
                match decode_result {
                    Ok(tuple) => Some((*rowid, tuple)),
                    Err(_) => None
                }
            },
            _ => None,
        }
    }
}

impl<'a> DbTableRange<'a> {
    pub fn new(iter: TableRange<'a>) -> DbTableRange {
        DbTableRange { range: iter }
    }
}

impl DbTable {
    pub fn new(schema: TupleSchema) -> DbTable {
        DbTable {
            table: Table::new(),
            schema: schema,
            next_rowid: 0,
        }
    }

    pub fn insert(&mut self, t: Tuple) {
        let serialized = t.serialize().unwrap();
        let rowid = self.next_rowid;
        self.next_rowid += 1;
        self.table.insert(rowid, serialized);
    }

    pub fn update(&mut self, r: RowId, t: Tuple) {
        self.delete(r);
        self.insert(t);
    }

    pub fn delete(&mut self, r: RowId) {
        self.table.delete(r);
    }

    pub fn cursor<'a>(&'a self) -> DbTableRange {
        DbTableRange::new(self.table.cursor())
    }
}

impl fmt::Debug for DbTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DbTable")
            .field("schema", &self.schema)
            .finish()
    }
}

#[test]
fn test_dbtable() {
    let mut table = DbTable::new(vec![TupleType::UnsignedInt32, TupleType::VarChar]);

    table.insert(vec![
        TupleVariant::UnsignedInt32(1),
        TupleVariant::VarChar("hello".to_owned()),
    ]);

    let mut c = table.cursor();

    let (k, v) = c.next().unwrap();

    assert_eq!(k, 0);
    assert_eq!(
        v,
        &[
            TupleVariant::UnsignedInt32(1),
            TupleVariant::VarChar("hello".to_owned())
        ]
    );
    assert!(c.next().is_none());
}
