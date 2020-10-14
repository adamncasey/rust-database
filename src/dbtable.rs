use crate::tuple::{Tuple, TupleSchema, TupleType, TupleVariant};
use std::collections::btree_map::Iter;
use std::collections::btree_map::Range;
use std::collections::BTreeMap;
use std::ops::Bound::*;
use std::fmt;

pub struct DbTable {
    rows: BTreeMap<RowId, Tuple>,
    schema: TupleSchema,
    next_rowid: RowId,
}

type DbTableIter<'a> = Iter<'a, RowId, Tuple>;
type DbTableRange<'a> = Range<'a, RowId, Tuple>;
type RowId = u32;

impl DbTable {
    pub fn new(schema: TupleSchema) -> DbTable {
        DbTable {
            rows: BTreeMap::new(),
            schema: schema,
            next_rowid: 0,
        }
    }

    pub fn insert(&mut self, t: Tuple) -> DbTableRange {
        let rowid = self.next_rowid;
        self.next_rowid += 1;
        self.rows.insert(rowid, t);
        self.rows.range((Included(rowid), Unbounded))
    }

    pub fn update(&mut self, r: RowId, t: Tuple) -> DbTableRange {
        self.delete(r);
        self.insert(t)
    }

    pub fn delete(&mut self, r: RowId) {
        self.rows.remove(&r);
    }

    pub fn cursor<'a>(&'a self) -> DbTableIter {
        self.rows.iter()
    }
}

impl fmt::Debug for DbTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DbTable").field("schema", &self.schema).finish()
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

    assert_eq!(*k, 0);
    assert_eq!(
        v,
        &[
            TupleVariant::UnsignedInt32(1),
            TupleVariant::VarChar("hello".to_owned())
        ]
    );
    assert!(c.next().is_none());
}
