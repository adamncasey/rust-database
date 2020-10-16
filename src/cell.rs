use bincode;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;

type CellBlock = [u8];

struct Cell {
    key_size: u32,                 // 4 bytes
    payload_size: u32,             // 4 bytes
    next_cell_offset: Option<u32>, // one byte + 4 bytes
}

const CELL_HEADER_SIZE: u32 = 4 + 4 + 1 + 4;

impl Cell {
    pub fn size(&self) -> u32 {
        self.key_size + self.payload_size + CELL_HEADER_SIZE
    }

    pub fn new(key_size: u32, payload_size: u32, next_cell_offset: Option<u32>) -> Cell {
        assert!(key_size > 0);
        assert!(payload_size > 0);

        Cell {
            key_size: key_size,
            payload_size: payload_size,
            next_cell_offset: next_cell_offset,
        }
    }

    pub fn new_from_memory(mem: &[u8], offset: u32) -> Result<Cell, &'static str> {
        if mem.len() < (offset + CELL_HEADER_SIZE).try_into().unwrap() {
            return Err("incorrect size");
        }

        let mut cursor = Cursor::new(mem);

        cursor.seek(SeekFrom::Start(offset.into())).unwrap();

        let key_size = cursor.read_u32::<BigEndian>().unwrap();
        let payload_size = cursor.read_u32::<BigEndian>().unwrap();
        let has_next = cursor.read_u8().unwrap();
        let next_cell_offset = if has_next != 0 {
            Some(cursor.read_u32::<BigEndian>().unwrap())
        } else {
            None
        };

        let cell = Cell::new(key_size, payload_size, next_cell_offset);

        if mem.len() < (offset + cell.size()).try_into().unwrap() {
            return Err("incorrect size");
        }

        Ok(cell)
    }

    pub fn save(&self, mem: &mut [u8], offset: u32) {
        let bytes = Self::range(offset, CELL_HEADER_SIZE, mem);

        let mut cursor = Cursor::new(mem);

        cursor.seek(SeekFrom::Start(offset.into())).unwrap();

        cursor.write_u32::<BigEndian>(self.key_size).unwrap();
        cursor.write_u32::<BigEndian>(self.payload_size).unwrap();
        cursor
            .write_u8(if self.next_cell_offset.is_some() {
                1
            } else {
                0
            })
            .unwrap();
        if self.next_cell_offset.is_some() {
            cursor
                .write_u32::<BigEndian>(self.next_cell_offset.unwrap())
                .unwrap();
        }
    }

    pub fn key<'a>(&self, mem: &'a mut [u8], cell_offset: u32) -> &'a mut [u8] {
        let start = cell_offset + CELL_HEADER_SIZE;

        Self::range(start, self.key_size, mem)
    }

    pub fn payload<'a>(&self, mem: &'a mut [u8], cell_offset: u32) -> &'a mut [u8] {
        let start = cell_offset + CELL_HEADER_SIZE + self.key_size;

        Self::range(start, self.payload_size, mem)
    }

    fn range<'a>(start: u32, length: u32, mem: &'a mut [u8]) -> &'a mut [u8] {
        let end = start + length;

        &mut mem[(start.try_into().unwrap())..(end.try_into().unwrap())]
    }
}

#[test]
fn cell_size() {
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

    let key_bytes = cell.key(&mut mem[..], 0);

    for byte in key_bytes {
        *byte = 7;
    }

    let payload_bytes = cell.payload(&mut mem[..], 0);

    for byte in payload_bytes {
        *byte = 8;
    }

    cell.save(&mut mem[..], 0);

    assert_eq!(
        mem[0..27],
        [0, 0, 0, 4, 0, 0, 0, 10, 1, 0, 0, 0, 12, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8]
    )
}
