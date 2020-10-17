use crate::cell::Cell;
use crate::cell::CELL_HEADER_SIZE;
use crate::cell::cell_size;

const LINKED_PAGE_HEADER: usize = 5 + 5 + 5;

struct LinkedPage {
    first_free_cell_offset: Option<usize>, // one byte + 4 bytes
    first_cell_offset: Option<usize>,      // one byte + 4 bytes
    next_page_no: Option<usize>,           // one byte + 4 bytes

    mem: Box<[u8]>,
}

struct CellIter<'a> {
    page: &'a LinkedPage,
    offset: Option<usize>,
}

impl<'a> Iterator for CellIter<'a> {
    type Item = (usize, Cell);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(offset) = self.offset {
            let cell = Cell::new_from_memory(&self.page.mem, offset).unwrap();
            self.offset = cell.next_cell();
            Some((offset, cell))
        } else {
            None
        }
    }
}

impl LinkedPage {
    pub fn new(size: usize) -> LinkedPage {
        let mem_size = size - LINKED_PAGE_HEADER;
        let mut mem = std::iter::repeat(0)
            .take(mem_size)
            .collect::<Vec<u8>>()
            .into_boxed_slice();
        let free_cell = Cell::new(mem_size - CELL_HEADER_SIZE, 0, None);
        let free_cell_offset = 0;
        free_cell.save(&mut mem, free_cell_offset as usize);

        LinkedPage {
            first_free_cell_offset: Some(free_cell_offset),
            first_cell_offset: None,
            next_page_no: None,

            mem: mem,
        }
    }

    pub fn storage_size(&self) -> usize {
        self.mem.len()
    }

    pub fn free_space(&self) -> usize {
        self.free_cell_iter().map(|(_, c)| c.size()).sum()
    }

    pub fn allocated_space(&self) -> usize {
        self.cell_iter().map(|(_, c)| c.size()).sum()
    }

    pub fn size(&self) -> usize {
        self.mem.len() + LINKED_PAGE_HEADER
    }

    fn free_cell_iter<'a>(&'a self) -> CellIter<'a> {
        CellIter {
            page: self,
            offset: self.first_free_cell_offset,
        }
    }

    pub fn cell_iter<'a>(&'a self) -> CellIter<'a> {
        CellIter {
            page: self,
            offset: self.first_cell_offset,
        }
    }

    pub fn validate_allocations(&self) -> Result<(usize, usize), &'static str> {
        let space = (self.allocated_space(), self.free_space());

        if space.0 + space.1 == self.storage_size() {
            Ok(space)
        } else {
            Err("allocation error")
        }
    }

    pub fn has_space_for(&self, key_size: usize, payload_size: usize) -> bool {
        cell_size(key_size, payload_size) <= self.free_space()
    }

    pub fn insert(&mut self, key: &[u8], payload: &[u8], after_cell: Option<usize>) -> Result<(), &'static str> {
        if !self.has_space_for(key.len(), payload.len()) {
            return Err("no space");
        }
        Err("Not implemented")
    }
}

#[test]
fn create_page() {
    let page = LinkedPage::new(1024);

    assert_eq!(page.storage_size(), 1024 - LINKED_PAGE_HEADER);
    assert_eq!(page.size(), 1024);

    assert_eq!(page.validate_allocations(), Ok((0, page.storage_size())));
}

#[test]
fn insert_cell() {
    let mut page = LinkedPage::new(1024);

    let after_cell = None;
    let insert_key = [1, 2, 3, 4];
    let insert_payload = [10, 10, 10, 10, 10, 10];

    assert!(page.has_space_for(insert_key.len(), insert_payload.len()));
    assert!(!page.has_space_for(1000, 24));

    page.insert(&insert_key, &insert_payload, after_cell).unwrap();

    let mut iter = page.cell_iter();

    let next = iter.next();
    assert!(next.is_some());
    let (offset, cell) = next.unwrap();
    let read_key = cell.key(&page.mem, offset);
    assert_eq!(read_key, insert_key);
    let read_payload = cell.payload(&page.mem, offset);
    assert_eq!(read_payload, insert_payload);
}
