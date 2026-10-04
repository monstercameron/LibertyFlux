//! Split of `lf-checker-worker` (lane k-split): JSON parsing and protocol helpers.
//! Move-only refactor of the single-file worker; behaviour unchanged.
use std::collections::HashMap;
use std::ffi::c_void;
use std::io::{BufRead, Write};
use std::sync::mpsc;
use std::time::Duration;
use crate::state::*;
use crate::image::*;
use crate::execution::*;
use crate::interception::*;
use crate::trial::*;


// ---------------------------------------------------------------------------
// Minimal JSON value + parser (zero dependencies) and emitter helpers.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) enum J {
    Null,
    Bool(bool),
    Int(i64),
    Str(String),
    Arr(Vec<J>),
    Obj(HashMap<String, J>),
}


impl J {
    pub(crate) fn get(&self, k: &str) -> Option<&J> {
        match self {
            J::Obj(m) => m.get(k),
            _ => None,
        }
    }
    pub(crate) fn as_i64(&self) -> i64 {
        match self {
            J::Int(i) => *i,
            J::Bool(b) => *b as i64,
            J::Str(s) => parse_u32(s) as i64,
            _ => 0,
        }
    }
    pub(crate) fn as_u32(&self) -> u32 {
        self.as_i64() as u32
    }
    pub(crate) fn as_usize(&self) -> usize {
        self.as_i64() as usize
    }
    pub(crate) fn as_str(&self) -> &str {
        match self {
            J::Str(s) => s,
            _ => "",
        }
    }
    pub(crate) fn as_bool(&self, d: bool) -> bool {
        match self {
            J::Bool(b) => *b,
            J::Int(i) => *i != 0,
            _ => d,
        }
    }
    pub(crate) fn as_arr(&self) -> &[J] {
        match self {
            J::Arr(a) => a,
            _ => &[],
        }
    }
}


pub(crate) fn parse_u32(t: &str) -> u32 {
    let t = t.trim();
    if let Some(x) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        u32::from_str_radix(x, 16).unwrap_or(0)
    } else if t.starts_with('-') {
        t.parse::<i32>().unwrap_or(0) as u32
    } else {
        t.parse::<u32>().unwrap_or(0)
    }
}


pub(crate) struct P<'a> {
    pub(crate) b: &'a [u8],
    pub(crate) p: usize,
}


pub(crate) fn parse_json(s: &str) -> Result<J, String> {
    let mut p = P {
        b: s.as_bytes(),
        p: 0,
    };
    let v = p.value()?;
    p.ws();
    if p.p != p.b.len() {
        return Err("trailing chars".to_string());
    }
    Ok(v)
}


impl<'a> P<'a> {
    pub(crate) fn ws(&mut self) {
        while self.p < self.b.len() && matches!(self.b[self.p], b' ' | b'\t' | b'\r' | b'\n') {
            self.p += 1;
        }
    }
    pub(crate) fn peek(&self) -> u8 {
        if self.p < self.b.len() {
            self.b[self.p]
        } else {
            0
        }
    }
    pub(crate) fn value(&mut self) -> Result<J, String> {
        self.ws();
        match self.peek() {
            b'{' => self.obj(),
            b'[' => self.arr(),
            b'"' => Ok(J::Str(self.string()?)),
            b't' => self.lit("true", J::Bool(true)),
            b'f' => self.lit("false", J::Bool(false)),
            b'n' => self.lit("null", J::Null),
            c if c == b'-' || c.is_ascii_digit() => self.num(),
            c => Err(format!("unexpected char {}", c)),
        }
    }
    pub(crate) fn lit(&mut self, s: &str, v: J) -> Result<J, String> {
        if self.b[self.p..].starts_with(s.as_bytes()) {
            self.p += s.len();
            Ok(v)
        } else {
            Err("bad literal".to_string())
        }
    }
    pub(crate) fn num(&mut self) -> Result<J, String> {
        let st = self.p;
        // ints, floats (floats are parsed here, not via the Int path)
        let mut is_float = false;
        while self.p < self.b.len()
            && (self.b[self.p].is_ascii_digit()
                || matches!(self.b[self.p], b'-' | b'+' | b'.' | b'e' | b'E'))
        {
            if matches!(self.b[self.p], b'.' | b'e' | b'E') {
                is_float = true;
            }
            self.p += 1;
        }
        let t = std::str::from_utf8(&self.b[st..self.p]).map_err(|_| "bad num")?;
        if is_float {
            // store floats scaled: keep as string in J::Str would lose type; use Int of bits? simplest: Int(f*1e12)
            let f: f64 = t.parse().map_err(|_| "bad float")?;
            Ok(J::Int((f * 1e12) as i64))
        } else {
            t.parse::<i64>()
                .map(J::Int)
                .map_err(|_| "bad int".to_string())
        }
    }
    pub(crate) fn string(&mut self) -> Result<String, String> {
        // assumes opening quote
        self.p += 1;
        let mut out = String::new();
        while self.p < self.b.len() {
            let c = self.b[self.p];
            self.p += 1;
            match c {
                b'"' => return Ok(out),
                b'\\' => {
                    if self.p >= self.b.len() {
                        break;
                    }
                    let e = self.b[self.p];
                    self.p += 1;
                    match e {
                        b'n' => out.push('\n'),
                        b't' => out.push('\t'),
                        b'r' => out.push('\r'),
                        b'u' => {
                            if self.p + 4 > self.b.len() {
                                return Err("bad \\u".to_string());
                            }
                            let h = std::str::from_utf8(&self.b[self.p..self.p + 4])
                                .map_err(|_| "bad \\u")?;
                            let cp = u32::from_str_radix(h, 16).map_err(|_| "bad \\u")?;
                            out.push(char::from_u32(cp).unwrap_or('?'));
                            self.p += 4;
                        }
                        _ => out.push(e as char),
                    }
                }
                _ => out.push(c as char),
            }
        }
        Err("unterminated string".to_string())
    }
    pub(crate) fn arr(&mut self) -> Result<J, String> {
        self.p += 1;
        let mut v = Vec::new();
        loop {
            self.ws();
            if self.peek() == b']' {
                self.p += 1;
                return Ok(J::Arr(v));
            }
            v.push(self.value()?);
            self.ws();
            match self.peek() {
                b',' => {
                    self.p += 1;
                }
                b']' => {
                    self.p += 1;
                    return Ok(J::Arr(v));
                }
                _ => return Err("bad array".to_string()),
            }
        }
    }
    pub(crate) fn obj(&mut self) -> Result<J, String> {
        self.p += 1;
        let mut m = HashMap::new();
        loop {
            self.ws();
            if self.peek() == b'}' {
                self.p += 1;
                return Ok(J::Obj(m));
            }
            if self.peek() != b'"' {
                return Err("bad obj key".to_string());
            }
            let k = self.string()?;
            self.ws();
            if self.peek() != b':' {
                return Err("bad obj colon".to_string());
            }
            self.p += 1;
            let v = self.value()?;
            m.insert(k, v);
            self.ws();
            match self.peek() {
                b',' => {
                    self.p += 1;
                }
                b'}' => {
                    self.p += 1;
                    return Ok(J::Obj(m));
                }
                _ => return Err("bad obj".to_string()),
            }
        }
    }
}


pub(crate) fn esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o
}


pub(crate) fn hx(v: u32) -> String {
    format!("\"0x{:x}\"", v)
}


pub(crate) fn hexbytes(b: &[u8]) -> String {
    const H: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(b.len() * 2);
    for &x in b {
        s.push(H[(x >> 4) as usize] as char);
        s.push(H[(x & 15) as usize] as char);
    }
    s
}


pub(crate) fn diag(msg: &str) {
    let _ = writeln!(std::io::stderr(), "worker: {}", msg);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::*;
    use crate::state::*;

    fn obj(s: &str) -> J {
        parse_json(s).expect("parse")
    }

    #[test]
    fn json_object_scalars() {
        let j = obj(r#"{"i":42,"n":-7,"s":"x","t":true,"f":false,"z":null}"#);
        assert_eq!(j.get("i").unwrap().as_u32(), 42);
        assert_eq!(j.get("n").unwrap().as_i64(), -7);
        assert_eq!(j.get("s").unwrap().as_str(), "x");
        assert!(j.get("t").unwrap().as_bool(false));
        assert!(!j.get("f").unwrap().as_bool(true));
        assert!(j.get("z").is_some());
        assert!(j.get("missing").is_none());
    }

    #[test]
    fn json_nesting_and_arrays() {
        let j = obj(r#"{"a":[1,2,{"b":[]}],"e":{}}"#);
        let a = j.get("a").unwrap().as_arr();
        assert_eq!(a.len(), 3);
        assert_eq!(a[0].as_u32(), 1);
        assert_eq!(a[2].get("b").unwrap().as_arr().len(), 0);
        assert!(j.get("e").unwrap().get("x").is_none());
    }

    #[test]
    fn json_string_escapes() {
        let j = obj(r#"{"s":"a\nb\t\"q\"\\A"}"#);
        assert_eq!(j.get("s").unwrap().as_str(), "a\nb\t\"q\"\\A");
    }

    #[test]
    fn json_float_scales() {
        let j = obj(r#"{"f":1.5}"#);
        assert_eq!(j.get("f").unwrap().as_i64(), 1_500_000_000_000);
    }

    #[test]
    fn json_errors() {
        assert!(parse_json("{bad").is_err());
        assert!(parse_json("[1,").is_err());
        assert!(parse_json("\"unterminated").is_err());
        assert!(parse_json("1 2").is_err());
        assert!(parse_json("").is_err());
        assert!(parse_json("{}").is_ok());
        assert!(parse_json("[]").is_ok());
        assert!(parse_json(" \t\r\n 7 ").is_ok());
    }

    #[test]
    fn scalar_helpers() {
        assert_eq!(parse_u32("0x10"), 16);
        assert_eq!(parse_u32("0Xff"), 255);
        assert_eq!(parse_u32("20"), 20);
        assert_eq!(parse_u32("-1"), 0xFFFF_FFFF);
        assert_eq!(parse_u32("nope"), 0);
        assert_eq!(J::Int(0).as_bool(true), false);
        assert_eq!(J::Int(5).as_bool(false), true);
        assert_eq!(J::Null.as_bool(true), true);
        assert_eq!(J::Null.as_arr().len(), 0);
        assert_eq!(J::Int(3).as_usize(), 3);
    }

    #[test]
    fn emitters() {
        assert_eq!(hx(0x1a2b), "\"0x1a2b\"");
        assert_eq!(hexbytes(&[0xAB, 0x00, 0xFF]), "ab00ff");
        assert_eq!(hexbytes(&[]), "");
        assert_eq!(esc("a\"b\\c\nd\x01"), "a\\\"b\\\\c\\nd\\u0001");
    }
}

