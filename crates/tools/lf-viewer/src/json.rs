//! A minimal JSON writer: just enough to emit a glTF document.
//!
//! Values are built as a [`Json`] tree and serialised compactly. Strings are
//! escaped per RFC 8259; numbers are written in Rust's shortest round-trip
//! form. JSON has no representation for non-finite numbers, so they are
//! written as `null`; the glTF builder never produces them (it rejects
//! geometry with non-finite positions before it gets here).

use std::fmt::Write as _;

/// A JSON value.
#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    /// `null`.
    Null,
    /// `true` or `false`.
    Bool(bool),
    /// An integer.
    Int(i64),
    /// A single-precision number (vertex data, factors).
    Num(f32),
    /// A string.
    Str(String),
    /// An array.
    Arr(Vec<Json>),
    /// An object; keys keep their insertion order.
    Obj(Vec<(String, Json)>),
}

impl Json {
    /// An object from `(key, value)` pairs.
    #[must_use]
    pub fn obj<const N: usize>(pairs: [(&str, Json); N]) -> Json {
        Json::Obj(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
    }

    /// A string value.
    #[must_use]
    pub fn str(s: impl Into<String>) -> Json {
        Json::Str(s.into())
    }

    /// An integer from any unsigned count or index.
    #[must_use]
    pub fn uint(n: usize) -> Json {
        Json::Int(i64::try_from(n).unwrap_or(i64::MAX))
    }

    /// An array of numbers.
    #[must_use]
    pub fn nums(values: &[f32]) -> Json {
        Json::Arr(values.iter().map(|&v| Json::Num(v)).collect())
    }

    /// Adds a key to an object (no effect on other values).
    pub fn push(&mut self, key: &str, value: Json) {
        if let Json::Obj(pairs) = self {
            pairs.push((key.to_string(), value));
        }
    }

    /// Serialises compactly.
    #[must_use]
    pub fn to_json_string(&self) -> String {
        let mut out = String::new();
        self.write(&mut out);
        out
    }

    fn write(&self, out: &mut String) {
        match self {
            Json::Null => out.push_str("null"),
            Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Json::Int(n) => {
                let _ = write!(out, "{n}");
            }
            Json::Num(v) => {
                if v.is_finite() {
                    let _ = write!(out, "{v}");
                } else {
                    out.push_str("null");
                }
            }
            Json::Str(s) => write_string(out, s),
            Json::Arr(items) => {
                out.push('[');
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    item.write(out);
                }
                out.push(']');
            }
            Json::Obj(pairs) => {
                out.push('{');
                for (i, (key, value)) in pairs.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    write_string(out, key);
                    out.push(':');
                    value.write(out);
                }
                out.push('}');
            }
        }
    }
}

/// Writes a quoted, escaped JSON string.
fn write_string(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if u32::from(c) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", u32::from(c));
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialises_every_kind() {
        let mut v = Json::obj([
            ("a", Json::Null),
            ("b", Json::Bool(true)),
            ("c", Json::Int(-3)),
            ("d", Json::nums(&[1.0, 0.5, -2.25])),
            ("e", Json::str("x\"y\\z\n\u{1}")),
        ]);
        v.push("f", Json::Arr(vec![]));
        let s = v.to_json_string();
        assert_eq!(
            s,
            r#"{"a":null,"b":true,"c":-3,"d":[1,0.5,-2.25],"e":"x\"y\\z\n\u0001","f":[]}"#
        );
        // An independent parser agrees.
        let parsed: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(parsed["e"], "x\"y\\z\n\u{1}");
        assert_eq!(parsed["d"][2], -2.25);
    }

    #[test]
    fn floats_round_trip_and_non_finite_is_null() {
        let v = Json::nums(&[0.1, 1.0e-7, 3.402_823_5e38, f32::NAN]);
        let s = v.to_json_string();
        let parsed: Vec<Option<f64>> = serde_json::from_str(&s).unwrap();
        #[allow(clippy::cast_possible_truncation)] // back to the f32 we wrote
        let back: Vec<Option<f32>> = parsed.iter().map(|x| x.map(|f| f as f32)).collect();
        assert_eq!(
            back,
            vec![Some(0.1), Some(1.0e-7), Some(3.402_823_5e38), None]
        );
    }
}
