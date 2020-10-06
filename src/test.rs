use crate::database::{Database};

const CREATE_TABLE_STATEMENT :&str = r#"
{
    "CreateTable":{
        "types":[
            {"name":"personid","data_type":"I32"},
            {"name":"age","data_type":"I32"},
            {"name":"name","data_type":{"FixedString": {"len":20}}}
        ]
    }
}
"#;

const SELECT_STATEMENT :&str = r#""Select""#;

#[test]
fn empty_table()
{
    let mut db = Database::new();

    db.json_query(&CREATE_TABLE_STATEMENT);

    let result = db.json_query(&SELECT_STATEMENT);

    assert_eq!(result, r#"{"num_rows":0,"rows":[],"comment":""}"#);
}

#[test]
fn insert_select()
{
    let mut db = Database::new();

    db.json_query(&CREATE_TABLE_STATEMENT);

    db.json_query(&r#"
        {
            "Insert": {
                "values": ["0", "20", "Alice"]
            }
        }
    "#);

    let result = db.json_query(&SELECT_STATEMENT);

    assert_eq!(result, r#"{"num_rows":1,"rows":[["0", "20", "Alice"]],"comment":""}"#);
}