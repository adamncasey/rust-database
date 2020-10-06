use crate::storage::MemoryStorage;

use std::convert::TryFrom;

mod Mem {
    use crate::storage::Page;
    use std::convert::TryFrom;
    use std::convert::TryInto;
    use std::ops::Deref;

    const ROW_HEADER_SIZE: usize = 4;

    pub fn read(page: &Page, offset: usize) -> u32 {
        let page_slice = &page[offset..offset + 4];
        u32::from_le_bytes(page_slice.try_into().expect("unable to read u32"))
    }

    pub fn get_row_count(page: &Page) -> u32 {
        read(page.deref(), 0)
    }

    pub fn get_row(page: &Page, row_no: u32, row_size: usize) -> &[u8] {
        let row_offset = usize::try_from(row_no).unwrap() * row_size + ROW_HEADER_SIZE;
        &page[row_offset..row_offset + row_size]
    }
}

#[derive(Debug)]
struct Tuple {}

struct Table {
    store: MemoryStorage,
    cols: Vec<ColumnTypes>,
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

        if self.cur_page == self.end_page {
            None
        } else {
            Some(self.table.read_row(self.cur_page, self.cur_row))
        }
    }
}

#[derive(Clone)]
enum ColumnTypes {
    UnsignedInt32,
    Varchar(u32),
}

impl ColumnTypes {
    pub fn size(&self) -> usize {
        match self {
            ColumnTypes::UnsignedInt32 => 4,
            ColumnTypes::Varchar(n) => usize::try_from(*n).unwrap(),
        }
    }
}

impl Table {
    pub fn new(cols: &[ColumnTypes]) -> Table {
        Table {
            store: MemoryStorage::new(),
            cols: cols.to_vec(),
        }
    }

    fn page_rows(&self, page_no: u32) -> u32 {
        let page = self.store.checkout(page_no);
        let page_size = Mem::get_row_count(&page);
        page_size
    }

    fn row_size(&self) -> usize {
        self.cols.iter().map(|c| c.size()).sum()
    }

    fn read_row(&mut self, page_no: u32, row_no: u32) -> Tuple {
        let row_slice = Mem::get_row(self.store.checkout(page_no), row_no, self.row_size());

        panic!("Not implemented")
    }

    fn write_row(&mut self, page_no: u32, row_no: u32, values: &Tuple) {
        panic!("Not implemented")
    }

    pub fn append(&self) {
        panic!("Not implemented")
    }
    pub fn update(&self) {
        panic!("Not implemented")
    }
    pub fn delete(&self) {
        panic!("Not implemented")
    }

    pub fn read(self) -> TableIter {
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

impl IntoIterator for Table {
    type Item = Tuple;
    type IntoIter = TableIter;

    fn into_iter(self) -> Self::IntoIter {
        self.read()
    }
}

#[test]
fn test_create_insert() {
    let mut t = Table::new(&[
        ColumnTypes::UnsignedInt32,
        ColumnTypes::UnsignedInt32,
        ColumnTypes::Varchar(4),
    ]);

    let mut iter = t.read();

    assert!(iter.next().is_none());
}
