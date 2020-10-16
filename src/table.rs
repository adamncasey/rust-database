use std::collections::btree_map::Range;
use std::collections::BTreeMap;

pub type TableCell = Box<[u8]>;

pub struct Table {
    rows: BTreeMap<RowId, TableCell>
}

pub type TableRange<'a> = Range<'a, RowId, TableCell>;
pub type RowId = u32;

impl Table {
    pub fn new() -> Table {
        Table {
            rows: BTreeMap::new()
        }
    }

    pub fn insert(&mut self, rowid: RowId, t: TableCell) {
        self.rows.insert(rowid, t);
    }

    pub fn delete(&mut self, r: RowId) {
        self.rows.remove(&r);
    }

    pub fn cursor<'a>(&'a self) -> TableRange {
        self.rows.range(..)
    }
}

#[test]
fn table() {
    let mut table = Table::new();

    table.insert(1, Box::new([1,2,3,4,5,6,7,8,9,0]));
    table.insert(2, Box::new([2,3,4,5,6,7,8,9,0,1]));

    let mut cursor = table.cursor();

    {
        let row = cursor.next();
        assert!(row.is_some());
        let (rowid, content) = row.unwrap();
        assert_eq!(*rowid, 1);
        assert_eq!(content.as_ref(), [1,2,3,4,5,6,7,8,9,0]);
    }

    {
        let row = cursor.next();
        assert!(row.is_some());
        let (rowid, content) = row.unwrap();
        assert_eq!(*rowid, 2);
        assert_eq!(content.as_ref(), [2,3,4,5,6,7,8,9,0,1]);
    }

    {
        let row = cursor.next();
        assert!(row.is_none());
    }
}