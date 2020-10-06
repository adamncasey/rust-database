0. tables are created (metadata loaded from file (JSON?))
1. planner plans "query"
    - take some representation of query and new up the required nodes into a graph
2. nodes use low level table api
3. low level table api uses storage api
4. storage api loads from disk / memory

graph node types:
- insert_node(source, destination)
- update_node(source, column_mapping)
- delete_node(source)
- filter_node(source, expression)
- table_scan_node(table)
- get_row_node(table, pk)
- literal_node(tuple)
- eval_node(source, expression)

example queries:
- simple insert row query
    - insert_node(literal_node(a1,a2,a3), tableA(A,B,C))
- simple select * from query
    - table_scan_node(tableA(A,B,C))
- simple select with filter
    - filter_node(table_scan_node(tableA(A,B,C)), "A==C")
- simple duplicate rows
    - insert_node(table_scan_node(tableA(A,B,C)))
- simple update
    - update_node(eval_node(get_row_node(tableA(A,B,C), "pk=1"), "D:=C*2"), "C:=D")
- simple delete
    - delete_node(get_row_node(tableA(A,B,C), "pk=1"))
- complex delete
    - delete_node(filter_node(table_scan_node(tableA(A,B,C), "A==2")))

nodes use storage api to get data:
- open_table
- table_append
- table_update
- table_delete
- table_read
- table_size
- sync

## example test implementation
- test:
```rust
let db = new Database()

db.create_table("A", [
    ["a", Int],
    ["b", Int],
    ["c", Str(4)]
]);

let a = TableScan("A")
let f = Filter(a, "a==4")

for row in f:
    println!("row: {:?}", row)
```

## example node implementation
```rust
struct TableScan {
    table: Table;
    cur: i32;
    begin: i32;
    end: i32;
}
impl TableScan {
    pub fn new(table_name: &str) {
        let t = open_table(table_name);
        TableScan {
            table: t,
            begin: 0,
            end: t.num_rows(),
            cur: 0
        }
    }

    pub fn next(): Result<Row> {
        if(cur == end) Err
        else table.get(cur)
        cur++
    }
}
```