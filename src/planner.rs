use sqlparser::ast::{ColumnDef, Expr, Statement, Value};
use sqlparser::parser::{Parser};
use sqlparser::dialect::{AnsiDialect};


#[derive(Debug)]
pub struct Plan {
    pub query: String,
    pub command: Statement,
}


pub fn column_names(columns: &[ColumnDef]) -> Vec<String> {
    columns.iter().map(|cd| cd.name.to_string()).collect()
}

pub fn expr_to_string(expr: &Expr) -> String {
    match expr {
        Expr::Value(v) => match v {
            Value::SingleQuotedString(s) => s.clone(),
            _ => v.to_string(),
        },
        _ => expr.to_string(),
    }
}

pub fn plan(input: &str) -> Result<Plan, &'static str> {
    let result = Parser::parse_sql(&AnsiDialect{}, input);
    let mut result = result.map_err(|parse_error| "parse error")?;

    match result.len() {
        0 => Err("no statements parsed"),
        1 => Ok(Plan{query: input.to_owned(), command: result.remove(0)}),
        _ => Err("too many statements")
    }
}
