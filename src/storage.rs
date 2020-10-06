use std::collections::HashMap;

struct MemoryStorage {
    pages: HashMap<u32, Box<Page>>,
    npages: u32,
}

pub type Page = [u8; 4096];

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
        self.pages.insert(page_num, Box::new([0; 4096]));
        page_num
    }

    pub fn delete(&mut self, num: u32) {
        self.pages.remove(&num).unwrap();
        self.npages -= 1;
    }

    pub fn checkout(&mut self, num: u32) -> Box<Page> {
        self.pages.remove(&num).unwrap()
    }

    pub fn retrn(&mut self, num: u32, page: Box<Page>) {
        self.pages.insert(num, page);
    }
}

#[test]
fn test_memstorage() {
    let mut store = MemoryStorage::new();

    assert_eq!(0, store.num_pages());

    let page_no = store.create();

    let mut page = store.checkout(page_no);

    page[0] = 0;
    page[1] = 1;
    page[2] = 2;

    store.retrn(page_no, page);

    assert_eq!(1, store.num_pages());

    let page = store.checkout(page_no);

    assert_eq!(page[0], 0);
    assert_eq!(page[1], 1);
    assert_eq!(page[2], 2);

    store.retrn(page_no, page);

    store.delete(page_no);

    assert_eq!(0, store.num_pages());
}
