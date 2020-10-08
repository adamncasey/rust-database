use std::convert::TryFrom;
use std::cmp;
use std::slice;

#[derive(Clone, Debug, PartialEq)]
pub enum ColumnType {
    UnsignedInt32,
    Varchar(usize),
}

impl ColumnType {
    pub fn size(&self) -> usize {
        match self {
            ColumnType::UnsignedInt32 => 4,
            ColumnType::Varchar(n) => usize::try_from(*n).unwrap(),
        }
    }

    pub fn serialize(&self, val: &str, dest: &mut [u8]) {
        assert_eq!(dest.len(), self.size());


        match self {
            ColumnType::UnsignedInt32 => {
                let int_val = val.parse::<u32>().unwrap_or(0);
                let bytes = int_val.to_le_bytes();
                dest.copy_from_slice(&bytes);
            },
            ColumnType::Varchar(n) => {
                let trimmed = &val[..cmp::min(val.len(), *n)];
                let (first, tail) = dest.split_at_mut(trimmed.len());
                first.copy_from_slice(trimmed.as_bytes());

                for extra in tail {
                    *extra = b' ';
                };
            }
        };
    }

    pub fn deserialize(&self, value: &[u8]) -> String {
        match self {
            ColumnType::UnsignedInt32 => {
                "ui32".to_owned()
            },
            ColumnType::Varchar(n) => {
                "vshar".to_owned()
            }
        }
    }
}

pub struct TupleSchema {
    cols: Vec<ColumnType>
}

impl TupleSchema {
    pub fn new(cols: &[ColumnType]) -> TupleSchema {
        TupleSchema {
            cols: cols.to_vec()
        }
    }

    pub fn size(&self) -> usize {
        self.cols.iter().map(|c| c.size()).sum()
    }

    pub fn len(&self) -> usize {
        self.cols.len()
    }

    pub fn serialize(&self, values: &[&str], dest: &mut [u8]) -> Result<(), &'static str> {
        if values.len() != self.cols.len() {
            return Err("invalid number of columns provided");
        }

        if dest.len() != self.size() {
            return Err("size mismatch");
        }

        let mut d = dest;
        for (col, val) in self.cols.iter().zip(values) {
            let (item, rest) = d.split_at_mut(col.size());

            col.serialize(val, item);

            d = rest;
        }

        assert_eq!(d.len(), 0);

        Ok(())
    }

    pub fn deserialize(&self, row: &[u8]) -> Result<Vec<String>, &'static str> {
        if row.len() != self.size() {
            return Err("size mismatch");
        }

        let mut results = vec![];
        let mut d = row;
        for col in self.cols.iter() {
            let (item, rest) = d.split_at(col.size());

            let decoded = col.deserialize(item);
            results.push(decoded);
            d = rest;
        };

        Ok(results)
    }
}
