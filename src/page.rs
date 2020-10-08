use std::convert::TryFrom;
use std::convert::TryInto;

// page module provides structured access to the content of a page

pub type Page = Box<[u8]>;

const ROW_HEADER_SIZE: usize = 4;

pub struct LeafPage {

}

impl LeafPage {
    fn read(page: &Page, offset: usize) -> u32 {
        let page_slice = &page[offset..offset + 4];
        assert_eq!(page_slice.len(), 4);

        u32::from_le_bytes(page_slice.try_into().expect("unable to read u32"))
    }

    pub fn get_row_count(page: &Page) -> u32 {
        Self::read(page, 0)
    }

    pub fn get_row(page: &Page, row_no: u32, row_size: usize) -> &[u8] {
        let row_offset = usize::try_from(row_no).unwrap() * row_size + ROW_HEADER_SIZE;
        &page[row_offset..row_offset + row_size]
    }

    fn write(page: &mut Page, offset: usize, count: u32) {
        let page_slice = &mut page[offset..offset + 4];
        assert_eq!(page_slice.len(), 4);

        let count_bytes = count.to_le_bytes();
        page_slice.copy_from_slice(&count_bytes);
    }

    pub fn set_row_count(page: &mut Page, count: u32) {
        Self::write(page, 0, count)
    }

    pub fn get_row_mut(page: &mut Page, row_no: u32, row_size: usize) -> &mut [u8] {
        let row_offset = usize::try_from(row_no).unwrap() * row_size + ROW_HEADER_SIZE;
        &mut page[row_offset..row_offset + row_size]
    }
}