use crate::tuple::{Tuple, TupleSchema, TupleType, TupleVariant};
use std::collections::BTreeMap;
use std::ops::Bound::*;

struct DbTable {
    rows: BTreeMap<RowId, Tuple>,
    schema: TupleSchema,
    next_rowid: RowId,
}

struct DbTableCursor<'a> {
    table: &'a DbTable,
    rowid: Option<RowId>,
}

type RowId = u32;

impl DbTable {
    pub fn new(schema: TupleSchema) -> DbTable {
        DbTable {
            rows: BTreeMap::new(),
            schema: schema,
            next_rowid: 0,
        }
    }

    pub fn insert(&mut self, t: Tuple) -> DbTableCursor {
        let rowid = self.next_rowid;
        self.next_rowid += 1;
        self.rows.insert(rowid, t);
        DbTableCursor::new(self, Some(rowid))
    }

    pub fn update(&mut self, r: RowId, t: Tuple) -> DbTableCursor {
        self.delete(r);
        self.insert(t)
    }

    pub fn delete(&mut self, r: RowId) {
        self.rows.remove(&r);
    }

    pub fn cursor<'a>(&'a self) -> DbTableCursor {
        DbTableCursor::new_from_start(self)
    }
}

impl DbTableCursor<'_> {
    pub fn new<'a>(table: &'a DbTable, r: Option<RowId>) -> DbTableCursor {
        DbTableCursor {
            table: table,
            rowid: r,
        }
    }

    pub fn new_from_start<'a>(table: &'a DbTable) -> DbTableCursor {
        let rowid= table.rows.keys().next();

        Self::new(table, match rowid {
            Some(r) => Some(*r),
            None => None
        })
    }

    pub fn has_row(&self) -> bool {
        self.rowid.is_some()
    }

    pub fn key(&self) -> RowId {
        self.rowid.unwrap()
    }

    pub fn value(&self) -> &Tuple {
        self.table.rows.get(&self.rowid.unwrap()).unwrap()
    }

    pub fn next(&mut self) -> bool {
        let mut range = self.table.rows.range((Excluded(self.rowid.unwrap()), Unbounded));

        match range.next() {
            Some((&r, _)) => {
                self.rowid = Some(r);
                true
            },
            None => {
                self.rowid = None;
                false
            }
        }
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

    assert!(c.has_row());
    assert_eq!(c.key(), 0);
    assert_eq!(
        c.value(),
        &[
            TupleVariant::UnsignedInt32(1),
            TupleVariant::VarChar("hello".to_owned())
        ]
    );
    assert!(!c.next());
    assert!(!c.has_row());
}
