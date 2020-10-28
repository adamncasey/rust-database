#[derive(Debug)]
pub struct Plan {
    query: String,
    pub operation: Operation,
}

#[derive(Debug)]
pub enum Operation {
    CreateTable {
        table_name: String,
        column_names: Vec<String>,
    },
    Insert {
        table_name: String,
        values: Vec<String>,
    },
    Select {
        table_name: String,
    },
    Unknown,
}
/**
 * CREATE TABLE persons (personid, name, nickname, favourite_takeaway)
 * rest: persons (personid, name, nickname, favourite_takeaway)
 */

fn error_plan(input: &str) -> Plan {
    Plan {
        query: input.to_string(),
        operation: Operation::Unknown,
    }
}

fn plan_create_table(input: &str) -> Option<Plan> {
    let rest = input.strip_prefix("CREATE TABLE");

    if None == rest {
        return None;
    }

    let rest = rest.unwrap().trim();

    let table_name = rest.split(" ").next();
    let paren_start = rest.find("(");
    let paren_end = rest.find(")");

    if table_name.is_none() || paren_start.is_none() || paren_end.is_none() {
        return Some(error_plan(input));
    }

    let input_cols = &rest[paren_start.unwrap() + 1..paren_end.unwrap()];

    let column_names = input_cols
        .split(",")
        .map(|col| col.trim().to_owned())
        .collect();

    return Some(Plan {
        query: input.to_string(),
        operation: Operation::CreateTable {
            table_name: table_name.unwrap().to_string(),
            column_names: column_names,
        },
    });
}

fn plan_insert(input: &str) -> Option<Plan> {
    let rest = input.strip_prefix("INSERT INTO");

    if None == rest {
        return None;
    }

    let rest = rest.unwrap().trim();

    let mut tokens = rest.split(" ");

    let table_name = match tokens.next() {
        Some(tname) => tname,
        None => return None,
    };

    tokens.next(); // skip 'VALUES'

    let start = input.find("(").unwrap() + 1;
    let end = input.find(")").unwrap();

    let values: Vec<String> = input[start..end]
        .split(",")
        .map(|val| val.trim().to_string())
        .collect();

    Some(Plan {
        query: input.to_string(),
        operation: Operation::Insert {
            table_name: table_name.to_string(),
            values: values,
        },
    })
}

pub fn plan(input: &str) -> Plan {
    if let Some(plan) = plan_create_table(input) {
        return plan;
    } else if let Some(plan) = plan_insert(input) {
        return plan;
    } else {
        return error_plan(input);
    }
}
