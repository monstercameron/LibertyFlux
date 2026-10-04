//! Native command names loaded from JSON at run time.
//!
//! The disassembler prints the hash operand of `NATIVE` as a name when a
//! [`NativeDb`] knows it. The table is never embedded: callers load the JSON
//! produced from the engine's native registration table (hash plus name per
//! entry) with [`NativeDb::from_p0_natives_json`].
//!
//! Expected JSON shape: an array of objects each carrying an integer `hash_int`
//! and a string `name`, with an optional `alias_names` array of extra strings.
//! Unknown fields are ignored; entries missing either field are skipped.

use std::collections::HashMap;

/// Hash-to-name table for native commands.
#[derive(Clone, Debug, Default)]
pub struct NativeDb {
    names: HashMap<u32, String>,
}

impl NativeDb {
    /// An empty table that resolves nothing.
    #[must_use]
    pub fn empty() -> Self {
        NativeDb::default()
    }

    /// Number of hashes in the table.
    #[must_use]
    pub fn len(&self) -> usize {
        self.names.len()
    }

    /// Whether the table holds no hashes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Build a table from the JSON array described in the module docs.
    ///
    /// Returns an error only when the text is not valid JSON or not an array;
    /// malformed entries inside a valid array are skipped.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn from_p0_natives_json(text: &str) -> Result<Self, DbError> {
        let value: serde_json::Value =
            serde_json::from_str(text).map_err(|e| DbError::BadJson(e.to_string()))?;
        let entries = value.as_array().ok_or(DbError::NotAnArray)?;
        let mut names = HashMap::new();
        for entry in entries {
            let Some(hash) = entry.get("hash_int").and_then(serde_json::Value::as_u64) else {
                continue;
            };
            let Ok(hash) = u32::try_from(hash) else {
                continue;
            };
            if let Some(name) = entry.get("name").and_then(serde_json::Value::as_str) {
                // Alias names share this same hash, so the primary name is all
                // a hash lookup needs; `alias_names` is accepted and ignored.
                names.insert(hash, name.to_string());
            }
        }
        Ok(NativeDb { names })
    }

    /// Look up a native hash. Returns the name or `None` when unknown.
    pub fn lookup(&self, hash: u32) -> Option<&str> {
        self.names.get(&hash).map(String::as_str)
    }
}

/// Errors from [`NativeDb::from_p0_natives_json`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DbError {
    /// The text is not valid JSON; carries the parser message.
    BadJson(String),
    /// The JSON is valid but not an array.
    NotAnArray,
}

impl core::fmt::Display for DbError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DbError::BadJson(msg) => write!(f, "native database is not valid JSON: {msg}"),
            DbError::NotAnArray => write!(f, "native database JSON must be an array"),
        }
    }
}

impl std::error::Error for DbError {}

#[cfg(test)]
mod tests {
    use super::*;

    /// Hand-written table in the expected shape; names are invented.
    const SAMPLE: &str = r#"[
        {"hash": "0x00000001", "hash_int": 1, "name": "FAKE_FIRST", "alias_names": ["FAKE_ALIAS"]},
        {"hash": "0x00000002", "hash_int": 2, "name": "FAKE_SECOND", "alias_names": []},
        {"hash_int": 3},
        {"name": "NO_HASH_HERE"}
    ]"#;

    #[test]
    fn loads_and_skips_bad_entries() {
        let db = NativeDb::from_p0_natives_json(SAMPLE).unwrap();
        assert_eq!(db.len(), 2);
        assert_eq!(db.lookup(1), Some("FAKE_FIRST"));
        assert_eq!(db.lookup(2), Some("FAKE_SECOND"));
        assert_eq!(db.lookup(3), None);
        assert_eq!(db.lookup(0xDEAD), None);
    }

    #[test]
    fn rejects_non_json_and_non_array() {
        assert!(matches!(
            NativeDb::from_p0_natives_json("nope"),
            Err(DbError::BadJson(_))
        ));
        assert_eq!(
            NativeDb::from_p0_natives_json(r#"{"a": 1}"#).unwrap_err(),
            DbError::NotAnArray
        );
    }
}
