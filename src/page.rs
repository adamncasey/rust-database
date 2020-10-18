use crate::cell::cell_size;
use crate::cell::Cell;
use crate::cell::CELL_HEADER_SIZE;

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

    pub fn allocate(&mut self, required_size: usize) -> Result<usize, &'static str> {
        // if no allocation possible, no changes are to be made.
        if required_size > self.free_space() {
            return Err("no space");
        }

        let free = self
            .free_cell_iter()
            .filter(|(_, c)| c.size() >= required_size)
            .next();

        // not enough space in one free cell? we need to defrag.
        assert!(free.is_some(), "defrag is required but not yet implemented");

        let (free_offset, free_cell) = free.unwrap();

        // temporarily as we deal with offsets to cells in the page, and not their indexes.
        // we have to loop over the cells again to find parents.
        let free_prev = self
            .free_cell_iter()
            .filter(|(o, c)| c.next_cell() == Some(free_offset))
            .next();

        if let Some((prev_free_offset, _)) = free_prev {
            // we can't iterate over mutable cells so construct a new mutable one.
            let mut prev_free_cell = Cell::new_from_memory(&self.mem, prev_free_offset).unwrap();
            // fixup free cell chain, ignoring the cell we removed.
            prev_free_cell.set_next_cell(free_cell.next_cell());
            prev_free_cell.save(&mut self.mem, prev_free_offset);
        } else {
            // there is no previous, update head of list.
            self.first_free_cell_offset = free_cell.next_cell();
        }

        // creating new free block

        let extra_size = free_cell.size() - required_size;

        if extra_size != 0 {
            assert!(
                extra_size >= CELL_HEADER_SIZE,
                "not yet handled cell fragments"
            );

            // create new free block to cover rest of space
            let extra_offset = free_offset + required_size;
            let extra_payload_size = extra_size - CELL_HEADER_SIZE;
            let extra_cell = Cell::new(0, extra_payload_size, self.first_free_cell_offset);
            extra_cell.save(&mut self.mem, extra_offset);

            // link in new free block to head of free list
            self.first_free_cell_offset = Some(extra_offset);
        }

        Ok(free_offset)
    }

    pub fn insert(
        &mut self,
        key: &[u8],
        payload: &[u8],
        after_cell: Option<usize>,
    ) -> Result<(), &'static str> {
        let required_size = cell_size(key.len(), payload.len());
        let new_cell_offset = self.allocate(required_size);

        if new_cell_offset.is_err() {
            return Err(new_cell_offset.unwrap_err());
        }

        let new_cell_offset = new_cell_offset.unwrap();

        // next cell offset if any
        // if user specified no after_cell then next cell is whatever was the prev first cell
        // else after cell is whatever was after the after_cell

        let next_cell_offset = if let Some(after_cell) = after_cell {
            Cell::new_from_memory(&self.mem, after_cell)
                .unwrap()
                .next_cell()
        } else {
            self.first_cell_offset
        };

        if after_cell.is_none() {
            // insert into first position
            self.first_cell_offset = Some(new_cell_offset);
        }

        let new_cell = Cell::new(key.len(), payload.len(), next_cell_offset);

        new_cell
            .key_mut(&mut self.mem, new_cell_offset)
            .copy_from_slice(key);
        new_cell
            .payload_mut(&mut self.mem, new_cell_offset)
            .copy_from_slice(payload);
        new_cell.save(&mut self.mem, new_cell_offset);

        Ok(())
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

    page.insert(&insert_key, &insert_payload, after_cell)
        .unwrap();

    page.validate_allocations().unwrap();

    let mut iter = page.cell_iter();

    let next = iter.next();
    assert!(next.is_some());
    let (offset, cell) = next.unwrap();
    let read_key = cell.key(&page.mem, offset);
    assert_eq!(read_key, insert_key);
    let read_payload = cell.payload(&page.mem, offset);
    assert_eq!(read_payload, insert_payload);

    let next = iter.next();

    assert!(next.is_none());
}
