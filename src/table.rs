use crate::storage::MemoryStorage;
use crate::tuple::{ColumnType, TupleSchema};
use crate::page::LeafPage;

struct Table {
    store: MemoryStorage,
    schema: TupleSchema,
}

struct TableIter<'a> {
    table: &'a Table,
    end_page: u32,
    cur_page: u32,
    end_row: u32,
    cur_row: u32,
}

impl<'a> Iterator for TableIter<'a> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
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

        if self.cur_page == self.end_page {
            None
        } else {
            Some(self.table.read_row(self.cur_page, self.cur_row))
        }
    }
}

impl Table {
    pub fn new(schema: TupleSchema) -> Table {
        Table {
            store: MemoryStorage::new(),
            schema: schema,
        }
    }

    fn page_rows(&self, page_no: u32) -> u32 {
        let page = self.store.checkout(page_no);
        let page_size = LeafPage::get_row_count(page);
        page_size
    }

    fn read_row(&self, page_no: u32, row_no: u32) -> Vec<String> {
        let page = self.store.checkout(page_no);
        let row = LeafPage::get_row(page, row_no, self.schema.size());
        self.schema.deserialize(row).unwrap()
    }

    pub fn read<'a>(&'a self) -> TableIter<'a> {
        let end_page = self.store.num_pages();

        let end_row = if end_page != 0 { self.page_rows(0) } else { 0 };

        TableIter {
            table: &self,
            end_page: end_page,
            end_row: end_row,
            cur_page: 0,
            cur_row: 0,
        }
    }

    fn page_full(&self, page_no: u32) -> bool {
        false
    }

    pub fn append(&mut self, values: &[&str]) -> Result<(), &'static str> {
        let last_page = match self.store.num_pages() {
            0 => self.store.create(),
            a => a,
        };

        let insert_page = if self.page_full(last_page) {
            self.store.create()
        } else {
            last_page
        };

        assert!(!self.page_full(insert_page));
        let page = self.store.checkout_mut(insert_page);

        let insert_row = LeafPage::get_row_count(page);

        let row = LeafPage::get_row_mut(page, insert_row, self.schema.size());

        match self.schema.serialize(values, row) {
            Ok(_) => {
                LeafPage::set_row_count(page, insert_row + 1);
                Ok(())
            },
            Err(msg) => Err(msg)
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

    let mut t = Table::new(tuple);

    t.append(&["1", "2", "abcd"]).unwrap();

    let mut iter = t.read();

    let opt_row = iter.next();

    assert!(opt_row.is_some());

    let row = opt_row.unwrap();

    assert_eq!(row.len(), 3);
    assert_eq!(row[0], "1");
    assert_eq!(row[1], "2");
    assert_eq!(row[2], "abcd");
}
