# `to_markdown_table`

An easy way to format any data structure into a Markdown table.

```toml
[dependencies]
to_markdown_table = "0.1.0"
```

## Example

```rust
use to_markdown_table::{MarkdownTable, TableRow};

struct User {
    name: String,
    age: u32
}

impl Into<TableRow> for &User {
    fn into(self) -> TableRow {
        TableRow::new(vec![self.name.clone(), self.age.to_string()])
    }
}

impl Into<TableRow> for User {
    fn into(self) -> TableRow {
        TableRow::new(vec![self.name.clone(), self.age.to_string()])
    }
}

let rows = vec![
    User { name: "Jessica".to_string(), age: 28 },
    User { name: "Dennis".to_string(), age: 22 }
];

let table = MarkdownTable::new(TableRow::new(vec!["Name".to_string(), "Age".to_string()]), rows).unwrap();

println!("{}", table);
```
