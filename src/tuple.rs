use bincode;
use serde::{Deserialize, Serialize};

pub type Tuple = Vec<TupleVariant>;
pub type TupleSchema = Vec<TupleType>;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum TupleVariant {
    UnsignedInt32(u32),
    SignedInt32(i32),
    Float32(f32),
    Float64(f64),
    VarChar(String),
    Null,
}

pub enum TupleType {
    UnsignedInt32,
    SignedInt32,
    Float32,
    Float64,
    VarChar,
}

trait ToStrings {
    fn to_strings(&self) -> Vec<String>;
}

trait FromString {
    fn from_string(&self, s: &str) -> Result<TupleVariant, &'static str>;
}

trait FromStrings {
    fn from_strings(strs: &[&str]) -> Self
    where
        Self: std::marker::Sized;
    fn from_strings_with_schema(strs: &[&str], schema: &TupleSchema) -> Result<Self, &'static str>
    where
        Self: std::marker::Sized;
}

impl ToString for TupleVariant {
    fn to_string(&self) -> String {
        match self {
            TupleVariant::UnsignedInt32(n) => n.to_string(),
            TupleVariant::SignedInt32(n) => n.to_string(),
            TupleVariant::Float32(f) => f.to_string(),
            TupleVariant::Float64(f) => f.to_string(),
            TupleVariant::VarChar(s) => s.clone(),
            TupleVariant::Null => "NULL".to_owned(),
        }
    }
}

impl ToStrings for Tuple {
    fn to_strings(&self) -> Vec<String> {
        self.iter().map(|i| i.to_string()).collect()
    }
}

impl FromString for TupleType {
    fn from_string(&self, s: &str) -> Result<TupleVariant, &'static str> {
        match self {
            TupleType::UnsignedInt32 => {
                let v = s.parse();
                if v.is_ok() {
                    Ok(TupleVariant::UnsignedInt32(v.unwrap()))
                } else {
                    Err("unable to convert to f64")
                }
            }
            TupleType::SignedInt32 => {
                let v = s.parse();
                if v.is_ok() {
                    Ok(TupleVariant::SignedInt32(v.unwrap()))
                } else {
                    Err("unable to convert to f64")
                }
            }
            TupleType::Float32 => {
                let v = s.parse();
                if v.is_ok() {
                    Ok(TupleVariant::Float32(v.unwrap()))
                } else {
                    Err("unable to convert to f64")
                }
            }
            TupleType::Float64 => {
                let v = s.parse();
                if v.is_ok() {
                    Ok(TupleVariant::Float64(v.unwrap()))
                } else {
                    Err("unable to convert to f64")
                }
            }
            TupleType::VarChar => Ok(TupleVariant::VarChar(s.to_string())),
        }
    }
}

impl FromStrings for Tuple {
    fn from_strings(strs: &[&str]) -> Tuple {
        strs.iter()
            .map(|s| TupleVariant::VarChar(s.to_string()))
            .collect()
    }

    fn from_strings_with_schema(
        strs: &[&str],
        schema: &TupleSchema,
    ) -> Result<Tuple, &'static str> {
        let converted: Vec<Result<TupleVariant, &'static str>> = strs
            .iter()
            .zip(schema)
            .map(|(s, t)| t.from_string(s))
            .collect();

        if converted.iter().any(|r| r.is_err()) {
            Err("error converting to tuple")
        } else {
            Ok(converted.into_iter().flatten().collect())
        }
    }
}

#[test]
fn test_to_strings() {
    let t = vec![
        TupleVariant::UnsignedInt32(1),
        TupleVariant::VarChar("hello".to_owned()),
        TupleVariant::Null,
    ];

    assert_eq!(t.to_strings(), ["1", "hello", "NULL"]);
}

#[test]
fn test_from_strings() {
    let t = Tuple::from_strings(&["1", "hello", "NULL"]);

    assert_eq!(t.len(), 3);
    assert_eq!(t[0], TupleVariant::VarChar("1".to_owned()));
    assert_eq!(t[1], TupleVariant::VarChar("hello".to_owned()));
    assert_eq!(t[2], TupleVariant::VarChar("NULL".to_owned()));
}

#[test]
fn test_from_string_with_schema() {
    let schema = vec![
        TupleType::SignedInt32,
        TupleType::VarChar,
        TupleType::Float64,
    ];
    let t = Tuple::from_strings_with_schema(&["1", "hello", "1.123"], &schema).unwrap();

    assert_eq!(t.len(), 3);
    assert_eq!(t[0], TupleVariant::SignedInt32(1));
    assert_eq!(t[1], TupleVariant::VarChar("hello".to_owned()));
    assert_eq!(t[2], TupleVariant::Float64(1.123));
}

#[test]
fn test_serialize() {
    let t = vec![
        TupleVariant::UnsignedInt32(1),
        TupleVariant::VarChar("hello".to_owned()),
        TupleVariant::Null,
    ];

    let encoded: Vec<u8> = bincode::serialize(&t).unwrap();

    assert_eq!(
        encoded,
        [
            3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0,
            104, 101, 108, 108, 111, 5, 0, 0, 0
        ]
    );

    let decoded: Tuple = bincode::deserialize(&encoded[..]).unwrap();

    assert_eq!(t.len(), decoded.len());
    for (l, r) in t.iter().zip(decoded.iter()) {
        assert_eq!(l, r);
    }
}
