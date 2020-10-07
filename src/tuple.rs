use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq)]
pub enum ColumnType {
    UnsignedInt32,
    Varchar(u32),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ColumnValue {
    UnsignedInt32(u32),
    Varchar(String),
}

impl ColumnType {
    pub fn size(&self) -> usize {
        match self {
            ColumnType::UnsignedInt32 => 4,
            ColumnType::Varchar(n) => usize::try_from(*n).unwrap(),
        }
    }
}

pub struct TupleSchema {
    cols: Vec<ColumnType>
}

pub type Tuple = Vec<ColumnValue>;

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
}
