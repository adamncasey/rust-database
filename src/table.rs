use crate::storage::MemoryStorage;
use crate::page;
use crate::tuple::{Tuple, TupleSchema, ColumnType, ColumnValue};

struct Table {
    store: MemoryStorage,
    schema: TupleSchema,
}

struct TableIter {
    table: Table,
    end_page: u32,
    cur_page: u32,
    end_row: u32,
    cur_row: u32,
}

impl Iterator for TableIter {
    type Item = Tuple;

    fn next(&mut self) -> Option<Tuple> {
        if self.cur_page != self.end_page {
            if self.cur_row == self.end_row {
                self.cur_row = 0;
                self.cur_page += 1;

                if self.cur_page == self.end_page {
                    self.end_row = 0;
                } else {
                    self.end_row = self.table.page_rows(self.cur_page);
                }
            } else {
                self.cur_row += 1
            };
        };

        //if self.cur_page == self.end_page {
            None
        //} else {
        //    Some(self.table.read_row(self.cur_page, self.cur_row))
        //}
    }
}

impl Table {
    pub fn new(schema: TupleSchema) -> Table {
        Table {
            store: MemoryStorage::new(),
            schema: schema,
        }
    }

    fn page_rows(&mut self, page_no: u32) -> u32 {
        let page = page::LeafPage::new(self.store.checkout(page_no));
        let page_size = page.get_row_count();
        page_size
    }

    pub fn read(mut self) -> TableIter {
        let end_page = self.store.num_pages();

        let end_row = if end_page != 0 { self.page_rows(0) } else { 0 };

        TableIter {
            table: self,
            end_page: end_page,
            end_row: end_row,
            cur_page: 0,
            cur_row: 0,
        }
    }
}

#[test]
fn test_create() {
    let t = Table::new(TupleSchema::new(&[
        ColumnType::UnsignedInt32,
        ColumnType::UnsignedInt32,
        ColumnType::Varchar(4),
    ]));

    let mut iter = t.read();

    assert!(iter.next().is_none());
}

#[test]
fn test_create_insert() {
    let tuple = TupleSchema::new(&[
        ColumnType::UnsignedInt32,
        ColumnType::UnsignedInt32,
        ColumnType::Varchar(4),
    ]);

    let t = Table::new(tuple);

    // t.append(&[1, 2, "abcd"]);

    let mut iter = t.read();

    let opt_row = iter.next();

    assert!(opt_row.is_some());

    let row = opt_row.unwrap();

    assert_eq!(row.len(), 3);
    assert_eq!(row[0], ColumnValue::UnsignedInt32(1));
    assert_eq!(row[1], ColumnValue::UnsignedInt32(2));
    assert_eq!(row[2], ColumnValue::Varchar("abcd".to_owned()));

}
