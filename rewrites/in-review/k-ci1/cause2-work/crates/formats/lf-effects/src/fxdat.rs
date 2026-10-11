//! The text `*.dat` effect rule tables.
//!
//! Every file opens with a version line, carries `#` comments (which hold
//! the column headers), and holds one or more `NAME_START` ... `NAME_END`
//! tables of whitespace-separated rows. The parser preserves versions,
//! table names and rows as strings; it does not interpret fields.

use crate::{Error, Result};

/// One `NAME_START` ... `NAME_END` table: a name plus string rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FxTable {
    /// Table name without the `_START` suffix (for example `EXPLOSIONFX_TABLE`).
    pub name: String,
    /// Data rows in file order; each row is the whitespace-split fields.
    pub rows: Vec<Vec<String>>,
}

impl FxTable {
    /// Number of rows.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// True when the table holds no rows.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Field count of the first row, if any.
    #[must_use]
    pub fn width(&self) -> Option<usize> {
        self.rows.first().map(Vec::len)
    }
}

/// A parsed rule file: a version plus its tables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FxFile {
    /// First non-empty, non-comment line (for example `1.00`).
    pub version: String,
    /// Tables in file order.
    pub tables: Vec<FxTable>,
}

impl FxFile {
    /// Parse rule-file text. Fails on rows outside a table, unclosed or
    /// mismatched table markers, nested tables, and a missing version.
    ///
    /// # Errors
    ///
    /// Returns an error on rows outside a table, unclosed or mismatched
    /// table markers, nested tables, or a missing version line.
    pub fn parse(text: &str) -> Result<FxFile> {
        let mut version: Option<String> = None;
        let mut tables: Vec<FxTable> = Vec::new();
        let mut open: Option<(String, Vec<Vec<String>>)> = None;
        for (idx, raw_line) in text.lines().enumerate() {
            let line_no = idx + 1;
            let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if version.is_none() {
                version = Some(trimmed.to_string());
                continue;
            }
            if let Some(name) = trimmed.strip_suffix("_START") {
                if open.is_some() {
                    return Err(Error::Malformed {
                        line: line_no,
                        detail: format!("nested table {name}"),
                    });
                }
                if name.is_empty() || name.contains(char::is_whitespace) {
                    return Err(Error::Malformed {
                        line: line_no,
                        detail: format!("bad table marker {trimmed:?}"),
                    });
                }
                open = Some((name.to_string(), Vec::new()));
                continue;
            }
            if let Some(name) = trimmed.strip_suffix("_END") {
                match open.take() {
                    Some((open_name, rows)) if open_name == name => {
                        tables.push(FxTable {
                            name: open_name,
                            rows,
                        });
                    }
                    Some((open_name, _)) => {
                        return Err(Error::Malformed {
                            line: line_no,
                            detail: format!("{name}_END closes {open_name}"),
                        });
                    }
                    None => {
                        return Err(Error::Malformed {
                            line: line_no,
                            detail: format!("{name}_END without a table"),
                        });
                    }
                }
                continue;
            }
            match open.as_mut() {
                Some((_, rows)) => {
                    rows.push(trimmed.split_whitespace().map(str::to_string).collect());
                }
                None => {
                    return Err(Error::Malformed {
                        line: line_no,
                        detail: format!("row outside any table: {trimmed:?}"),
                    });
                }
            }
        }
        if let Some((name, _)) = open {
            return Err(Error::Malformed {
                line: 0,
                detail: format!("table {name} never closed"),
            });
        }
        let Some(version) = version else {
            return Err(Error::Malformed {
                line: 0,
                detail: "missing version line".to_string(),
            });
        };
        Ok(FxFile { version, tables })
    }

    /// Find a table by name, if present.
    #[must_use]
    pub fn table(&self, name: &str) -> Option<&FxTable> {
        self.tables.iter().find(|t| t.name == name)
    }

    /// Total rows across all tables.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.tables.iter().map(FxTable::len).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Hand-written fixture in the shipped shape; no game content.
    const FIXTURE: &str = "1.0\r\n\r\n# TYPE\tFX_NAME\r\n\r\nSAMPLE_TABLE_START\r\n\r\n\
        SPARK\t\tspark_small\t1.0\r\nFLAME\t\tflame_big\t2.0\r\n\r\nSAMPLE_TABLE_END\r\n";

    #[test]
    fn parses_two_tables() {
        let text = "2.00\n# comment\nFIRST_START\nA B\nFIRST_END\nSECOND_START\nC\nSECOND_END\n";
        let file = FxFile::parse(text).unwrap();
        assert_eq!(file.version, "2.00");
        assert_eq!(file.tables.len(), 2);
        assert_eq!(file.row_count(), 2);
        assert_eq!(file.table("FIRST").unwrap().width(), Some(2));
        assert_eq!(file.table("MISSING"), None);
    }

    #[test]
    fn parses_crlf_fixture() {
        let file = FxFile::parse(FIXTURE).unwrap();
        assert_eq!(file.version, "1.0");
        assert_eq!(file.tables.len(), 1);
        let table = &file.tables[0];
        assert_eq!(table.name, "SAMPLE_TABLE");
        assert_eq!(
            table.rows,
            vec![
                vec!["SPARK", "spark_small", "1.0"],
                vec!["FLAME", "flame_big", "2.0"],
            ]
        );
    }

    #[test]
    fn rejects_malformed_files() {
        // Row outside a table.
        assert!(FxFile::parse("1.0\nSTRAY ROW\nA_START\nx\nA_END\n").is_err());
        // Mismatched end marker.
        assert!(FxFile::parse("1.0\nA_START\nx\nB_END\n").is_err());
        // Nested table.
        assert!(FxFile::parse("1.0\nA_START\nB_START\nx\nB_END\nA_END\n").is_err());
        // Unclosed table.
        assert!(FxFile::parse("1.0\nA_START\nx\n").is_err());
        // Empty file.
        assert!(FxFile::parse("").is_err());
        assert!(FxFile::parse("# only a comment\n").is_err());
    }
}
