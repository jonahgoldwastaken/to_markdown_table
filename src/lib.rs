//! A simple library to create markdown tables.
//!
//! ## Example
//!
//! ```rust
//! use to_markdown_table::{MarkdownTable, TableRow};
//!
//! struct User {
//!     name: String,
//!     age: u32
//! }
//!
//! impl Into<TableRow> for &User {
//!     fn into(self) -> TableRow {
//!         TableRow::new(vec![self.name.clone(), self.age.to_string()])
//!     }
//! }
//!
//! impl Into<TableRow> for User {
//!     fn into(self) -> TableRow {
//!         TableRow::new(vec![self.name.clone(), self.age.to_string()])
//!     }
//! }
//!
//! let rows = vec![
//!     User { name: "Jessica".to_string(), age: 28 },
//!     User { name: "Dennis".to_string(), age: 22 }
//! ];
//!
//! let table = MarkdownTable::new(TableRow::new(vec!["Name".to_string(), "Age".to_string()]), rows).unwrap();
//!
//! println!("{}", table);
//! ```

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MarkdownTableError {
    #[error("Invalid row length, expected {0} got {1}.")]
    InvalidRowLength(usize, usize),

    #[error("Length of rows must be at least 1 when creating a table.")]
    NoRowsSpecified,
}

type Result<T> = std::result::Result<T, MarkdownTableError>;

pub struct MarkdownTable {
    header: TableRow,
    rows: Vec<TableRow>,
}

impl std::fmt::Display for MarkdownTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for col in 0..self.cols() {
            let len = self.col_len(col).unwrap_or(0);
            write!(
                f,
                "| {text:width$} ",
                text = self.header.0[col],
                width = len
            )?;
        }
        writeln!(f, "|")?;

        for col in 0..self.cols() {
            let len = self.col_len(col).unwrap_or(0);
            write!(f, "| {text} ", text = "-".repeat(len))?;
        }
        writeln!(f, "|")?;

        for row in &self.rows {
            for col in 0..self.cols() {
                let len = self.col_len(col).unwrap_or(0);
                write!(f, "| {text:width$} ", text = row.0[col], width = len)?;
            }
            writeln!(f, "|")?;
        }

        Ok(())
    }
}

impl MarkdownTable {
    pub fn new(header: TableRow, rows: Vec<impl Into<TableRow>>) -> Result<Self> {
        let rows: Vec<TableRow> = rows.into_iter().map(|v| v.into()).collect();

        for row in &rows {
            Self::validate_row_length(&header, row)?;
        }

        Ok(Self { header, rows })
    }

    pub fn add_row(&mut self, row: impl Into<TableRow>) -> Result<()> {
        let row = row.into();
        Self::validate_row_length(&self.header, &row)?;
        self.rows.push(row);

        Ok(())
    }

    fn cols(&self) -> usize {
        self.header.0.len()
    }

    fn col_len(&self, col: usize) -> Option<usize> {
        if col >= self.cols() {
            None
        } else {
            let col_len = self.rows.iter().fold(0, |acc, curr| {
                if curr.col_len(col) > acc {
                    curr.col_len(col)
                } else {
                    acc
                }
            });

            if col_len > self.header.0[col].len() {
                Some(col_len)
            } else {
                Some(self.header.0[col].len())
            }
        }
    }

    fn validate_row_length(header: &TableRow, new_row: &TableRow) -> Result<()> {
        let header_len = header.0.len();
        let new_len = new_row.0.len();

        if header_len != new_len {
            Err(MarkdownTableError::InvalidRowLength(header_len, new_len))
        } else {
            Ok(())
        }
    }
}

pub struct TableRow(Vec<String>);

impl TableRow {
    pub fn new(data: Vec<String>) -> Self {
        Self(data)
    }

    fn col_len(&self, col: usize) -> usize {
        self.0[col].len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct DummyRow {
        data: Vec<String>,
    }

    impl From<DummyRow> for TableRow {
        fn from(val: DummyRow) -> Self {
            TableRow(val.data)
        }
    }

    impl From<&DummyRow> for TableRow {
        fn from(val: &DummyRow) -> Self {
            TableRow(val.data.clone())
        }
    }

    #[test]
    fn adding_row() {
        let dd = DummyRow {
            data: vec!["a".to_string(), "b".to_string()],
        };

        let mut mt = MarkdownTable::new(
            TableRow(vec!["Hoi".to_string(), "Bye".to_string()]),
            vec![dd],
        )
        .unwrap();

        let res = mt.add_row(DummyRow {
            data: vec!["c".to_string(), "d".to_string()],
        });

        assert!(res.is_ok());
    }

    #[test]
    fn wrong_row_size() {
        let dd = DummyRow {
            data: vec!["a".to_string(), "b".to_string()],
        };

        let mut mt = MarkdownTable::new(
            TableRow(vec!["Hoi".to_string(), "Bye".to_string()]),
            vec![dd],
        )
        .unwrap();

        let res = mt.add_row(DummyRow {
            data: vec!["d".to_string()],
        });

        assert!(res.is_err());
    }

    #[test]
    fn to_string() {
        let dd = vec![
            DummyRow {
                data: vec!["a".to_string(), "b".to_string()],
            },
            DummyRow {
                data: vec!["c".to_string(), "d".to_string()],
            },
            DummyRow {
                data: vec!["e".to_string(), "fg".to_string()],
            },
        ];

        let mt =
            MarkdownTable::new(TableRow(vec!["Hoi".to_string(), "Bye".to_string()]), dd).unwrap();

        println!("{}", mt);
    }
}
