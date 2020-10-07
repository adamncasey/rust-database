use std::convert::TryFrom;
use std::convert::TryInto;

// page module provides structured access to the content of a page

pub type Page = Box<[u8]>;

pub struct LeafPage<'a> {
    pub page: &'a Page,
}

impl LeafPage<'_> {
    const ROW_HEADER_SIZE: usize = 4;

    pub fn new(page: &Page) -> LeafPage {
        LeafPage {
            page: page
        }
    }

    fn read(&self, offset: usize) -> u32 {
        let page_slice = &self.page[offset..offset + 4];
        u32::from_le_bytes(page_slice.try_into().expect("unable to read u32"))
    }

    pub fn get_row_count(&self) -> u32 {
        self.read(0)
    }

    pub fn get_row(&self, row_no: u32, row_size: usize) -> &[u8] {
        let row_offset = usize::try_from(row_no).unwrap() * row_size + LeafPage::ROW_HEADER_SIZE;
        &self.page[row_offset..row_offset + row_size]
    }
}