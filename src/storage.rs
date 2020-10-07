use std::collections::HashMap;
use crate::page::Page;

pub struct MemoryStorage {
    pages: HashMap<u32, Page>,
    npages: u32,
}

impl MemoryStorage {
    pub fn new() -> MemoryStorage {
        MemoryStorage {
            pages: HashMap::new(),
            npages: 0,
        }
    }

    pub fn num_pages(&self) -> u32 {
        self.npages
    }

    pub fn create(&mut self) -> u32 {
        let page_num = self.npages;
        self.npages += 1;
        let new_page = Box::new([0; 4096]);
        self.pages.insert(page_num, new_page);
        page_num
    }

    pub fn checkout(&mut self, num: u32) -> &Page {
        self.pages.get(&num).unwrap()
    }

    pub fn checkout_mut(&mut self, num: u32) -> &mut Page {
        self.pages.get_mut(&num).unwrap()
    }
}

#[test]
fn test_memstorage() {
    let mut store = MemoryStorage::new();

    assert_eq!(0, store.num_pages());

    let page_no = store.create();

    assert_eq!(1, store.num_pages());

    {
        let page = store.checkout_mut(page_no);

        page[0] = 0;
        page[1] = 1;
        page[2] = 2;
    }

    {
        let page = store.checkout(page_no);

        assert_eq!(page[0], 0);
        assert_eq!(page[1], 1);
        assert_eq!(page[2], 2);
    }
}
