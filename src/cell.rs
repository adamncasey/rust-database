use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;

pub struct Cell {
    key_size: usize,                 // 4 bytes
    payload_size: usize,             // 4 bytes
    next_cell_offset: Option<usize>, // one byte + 4 bytes
}

pub const CELL_HEADER_SIZE: usize = 4 + 4 + 1 + 4;

pub fn cell_size(key_size: usize, payload_size: usize) -> usize {
    key_size + payload_size + CELL_HEADER_SIZE
}

impl Cell {
    pub fn size(&self) -> usize {
        cell_size(self.key_size, self.payload_size)
    }

    pub fn new(key_size: usize, payload_size: usize, next_cell_offset: Option<usize>) -> Cell {
        Cell {
            key_size: key_size,
            payload_size: payload_size,
            next_cell_offset: next_cell_offset,
        }
    }

    pub fn next_cell(&self) -> Option<usize> {
        self.next_cell_offset
    }

    pub fn set_next_cell(&mut self, new_next: Option<usize>) {
        self.next_cell_offset = new_next;
    }

    pub fn new_from_memory(mem: &[u8], offset: usize) -> Result<Cell, &'static str> {
        if mem.len() < offset + CELL_HEADER_SIZE {
            return Err("incorrect size");
        }

        let mut cursor = Cursor::new(mem);

        cursor.seek(SeekFrom::Start(offset as u64)).unwrap();

        let key_size = cursor.read_u32::<BigEndian>().unwrap();
        let payload_size = cursor.read_u32::<BigEndian>().unwrap();
        let has_next = cursor.read_u8().unwrap();
        let next_cell_offset = if has_next != 0 {
            Some(cursor.read_u32::<BigEndian>().unwrap() as usize)
        } else {
            None
        };

        let cell = Cell::new(key_size as usize, payload_size as usize, next_cell_offset);

        if mem.len() < (offset + cell.size()).try_into().unwrap() {
            return Err("incorrect size");
        }

        Ok(cell)
    }

    pub fn save(&self, mem: &mut [u8], offset: usize) {
        let bytes = Self::range(offset, CELL_HEADER_SIZE, mem);

        let mut cursor = Cursor::new(mem);

        cursor.seek(SeekFrom::Start(offset as u64)).unwrap();

        cursor.write_u32::<BigEndian>(self.key_size as u32).unwrap();
        cursor.write_u32::<BigEndian>(self.payload_size as u32).unwrap();
        cursor
            .write_u8(if self.next_cell_offset.is_some() {
                1
            } else {
                0
            })
            .unwrap();
        if self.next_cell_offset.is_some() {
            cursor
                .write_u32::<BigEndian>(self.next_cell_offset.unwrap() as u32)
                .unwrap();
        }
    }

    pub fn key<'a>(&self, mem: &'a [u8], cell_offset: usize) -> &'a [u8] {
        let start = cell_offset + CELL_HEADER_SIZE;

        Self::range(start, self.key_size, mem)
    }

    pub fn payload<'a>(&self, mem: &'a [u8], cell_offset: usize) -> &'a [u8] {
        let start = cell_offset + CELL_HEADER_SIZE + self.key_size;

        Self::range(start, self.payload_size, mem)
    }

    pub fn key_mut<'a>(&self, mem: &'a mut [u8], cell_offset: usize) -> &'a mut [u8] {
        let start = cell_offset + CELL_HEADER_SIZE;

        Self::range_mut(start, self.key_size, mem)
    }

    pub fn payload_mut<'a>(&self, mem: &'a mut [u8], cell_offset: usize) -> &'a mut [u8] {
        let start = cell_offset + CELL_HEADER_SIZE + self.key_size;

        Self::range_mut(start, self.payload_size, mem)
    }

    fn range<'a>(start: usize, length: usize, mem: &'a [u8]) -> &'a [u8] {
        let end = start + length;

        &mem[(start..end)]
    }

    fn range_mut<'a>(start: usize, length: usize, mem: &'a mut [u8]) -> &'a mut [u8] {
        let end = start + length;

        &mut mem[(start..end)]
    }
}

#[test]
fn test_cell_size() {
    let cell = Cell::new(4, 124, None);

    assert_eq!(cell.size(), 124 + 4 + CELL_HEADER_SIZE);
}

#[test]
fn cell_from_mem() {
    //  key size |payload size n? next offset |the key ----|the payload ----------------|
    //  .         .            .  .            .            .
    //  kkkkkkkkkkppppppppppppphhhnnnnnnnnnnnnnKKKKKKKKKKKKKPPPPPPPPPPPPPPPPPPPPPPPPPPPPP
    let mut mem = vec![
        0, 0, 0, 4, 0, 0, 0, 10, 1, 0, 0, 0, 12, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    ];

    let cell = Cell::new_from_memory(&mem[..], 0).unwrap();

    assert_eq!(cell.size(), 10 + 4 + CELL_HEADER_SIZE);
    assert_eq!(cell.key(&mut mem[..], 0), [7, 7, 7, 7]);
    assert_eq!(cell.payload(&mut mem[..], 0), [8, 8, 8, 8, 8, 8, 8, 8, 8, 8]);
}

#[test]
fn cell_to_mem() {
    let cell = Cell::new(4, 10, Some(12));

    let mut mem: Vec<u8> = vec![0; 128];

    let key_bytes = cell.key_mut(&mut mem[..], 0);

    for byte in key_bytes {
        *byte = 7;
    }

    let payload_bytes = cell.payload_mut(&mut mem[..], 0);

    for byte in payload_bytes {
        *byte = 8;
    }

    cell.save(&mut mem[..], 0);

    assert_eq!(
        mem[0..27],
        [0, 0, 0, 4, 0, 0, 0, 10, 1, 0, 0, 0, 12, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8]
    )
}
