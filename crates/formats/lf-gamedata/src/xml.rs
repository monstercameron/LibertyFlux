//! Minimal XML reader for the game's data XML files.
//!
//! The shipped XML (`WeaponInfo.xml`, `ThrownWeaponInfo.xml`,
//! `frontend_menus.xml`, `leaderboards_data.xml`, rain emitter/render
//! files) uses a small subset of XML: elements with double-quoted
//! attributes, `<!-- -->` comments, self-closing tags, no namespaces, no
//! entities beyond the predefined five, no CDATA, no processing
//! instructions. This parser accepts exactly that subset and reports
//! anything else as an error. It is deliberately not a general XML parser.

use crate::{Error, ErrorKind, Result, decode};

/// One XML element: name, attributes, children, text.
#[derive(Debug, Clone, Default)]
pub struct Element {
    /// Element name.
    pub name: String,
    /// Attributes in file order.
    pub attrs: Vec<(String, String)>,
    /// Child elements.
    pub children: Vec<Element>,
    /// Trimmed text content (concatenated), if any.
    pub text: Option<String>,
}

impl Element {
    /// Attribute value by name, if present.
    #[must_use]
    pub fn attr(&self, name: &str) -> Option<&str> {
        self.attrs
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }

    /// Child elements with this name.
    pub fn children_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Element> + 'a {
        self.children.iter().filter(move |c| c.name == name)
    }

    /// First child with this name.
    #[must_use]
    pub fn child(&self, name: &str) -> Option<&Element> {
        self.children.iter().find(|c| c.name == name)
    }

    /// Parse an attribute as `f32`.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn attr_f32(&self, file: &str, name: &str) -> Result<Option<f32>> {
        match self.attr(name) {
            None => Ok(None),
            Some(v) => v.parse::<f32>().map(Some).map_err(|_| {
                Error::whole_file(
                    file,
                    ErrorKind::BadHeader {
                        want: "numeric attribute",
                    },
                )
            }),
        }
    }
}

/// Deepest element nesting accepted. The game's XML files nest a handful
/// of levels; the cap keeps hostile input from overflowing the stack, since
/// elements are parsed recursively (found by the fuzz tests).
const MAX_DEPTH: usize = 256;

struct Cursor<'a> {
    b: &'a [u8],
    pos: usize,
    file: &'a str,
    /// Elements currently open (recursion depth of [`Cursor::element`]).
    depth: usize,
}

impl Cursor<'_> {
    fn eof(&self) -> bool {
        self.pos >= self.b.len()
    }

    fn peek(&self) -> Option<u8> {
        self.b.get(self.pos).copied()
    }

    fn starts_with(&self, s: &[u8]) -> bool {
        self.b.len() - self.pos >= s.len() && &self.b[self.pos..self.pos + s.len()] == s
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\r' | b'\n')) {
            self.pos += 1;
        }
    }

    // Error-path line counting only; the bytecount crate is not worth a
    // dependency for this.
    #[allow(clippy::naive_bytecount)]
    fn err(&self, want: &'static str) -> Error {
        // Best-effort line number by counting newlines.
        let line = self.b[..self.pos.min(self.b.len())]
            .iter()
            .filter(|&&c| c == b'\n')
            .count()
            + 1;
        Error::new(file_of(self.file), line, ErrorKind::BadHeader { want })
    }

    fn expect(&mut self, c: u8, want: &'static str) -> Result<()> {
        if self.peek() == Some(c) {
            self.pos += 1;
            Ok(())
        } else {
            Err(self.err(want))
        }
    }

    fn name(&mut self) -> Result<String> {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || matches!(c, b'_' | b'-' | b':' | b'.') {
                self.pos += 1;
            } else {
                break;
            }
        }
        if self.pos == start {
            return Err(self.err("element or attribute name"));
        }
        Ok(String::from_utf8_lossy(&self.b[start..self.pos]).into_owned())
    }

    fn element(&mut self) -> Result<Element> {
        if self.depth >= MAX_DEPTH {
            return Err(self.err("element nesting within the depth limit"));
        }
        self.depth += 1;
        let el = self.element_body();
        self.depth -= 1;
        el
    }

    fn element_body(&mut self) -> Result<Element> {
        self.expect(b'<', "`<`")?;
        if self.peek() == Some(b'/') || self.peek() == Some(b'!') || self.peek() == Some(b'?') {
            return Err(self.err("element start"));
        }
        let name = self.name()?;
        let mut el = Element {
            name,
            ..Element::default()
        };
        loop {
            self.skip_ws();
            match self.peek() {
                None => return Err(self.err("`>` or `/>`")),
                Some(b'/') => {
                    self.pos += 1;
                    self.expect(b'>', "`/>`")?;
                    return Ok(el);
                }
                Some(b'>') => {
                    self.pos += 1;
                    break;
                }
                Some(_) => {
                    let key = self.name()?;
                    self.skip_ws();
                    self.expect(b'=', "`=`")?;
                    self.skip_ws();
                    let quote = self.peek();
                    if quote != Some(b'"') && quote != Some(b'\'') {
                        return Err(self.err("quoted attribute value"));
                    }
                    self.pos += 1;
                    let start = self.pos;
                    while self.peek() != quote {
                        if self.eof() {
                            return Err(self.err("closing quote"));
                        }
                        self.pos += 1;
                    }
                    let raw = String::from_utf8_lossy(&self.b[start..self.pos]).into_owned();
                    self.pos += 1;
                    el.attrs.push((key, unescape(&raw)));
                }
            }
        }
        // Children / text / close tag.
        let mut text = String::new();
        loop {
            self.skip_ws();
            if self.eof() {
                return Err(self.err("closing tag"));
            }
            if self.starts_with(b"</") {
                self.pos += 2;
                let close = self.name()?;
                self.skip_ws();
                self.expect(b'>', "`>`")?;
                if close != el.name {
                    return Err(self.err("matching closing tag"));
                }
                let t = text.trim();
                if !t.is_empty() {
                    el.text = Some(t.to_string());
                }
                return Ok(el);
            }
            if self.starts_with(b"<!--") {
                self.pos += 4;
                loop {
                    if self.eof() {
                        return Err(self.err("`-->`"));
                    }
                    if self.starts_with(b"-->") {
                        self.pos += 3;
                        break;
                    }
                    self.pos += 1;
                }
                continue;
            }
            if self.peek() == Some(b'<') {
                el.children.push(self.element()?);
            } else {
                let start = self.pos;
                while !self.eof() && self.peek() != Some(b'<') {
                    self.pos += 1;
                }
                text.push_str(&String::from_utf8_lossy(&self.b[start..self.pos]));
            }
        }
    }
}

fn file_of(f: &str) -> &str {
    f
}

/// Decode the five predefined XML entities.
fn unescape(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

/// Parse one XML document from bytes, returning its root element.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_xml(file: &str, bytes: &[u8]) -> Result<Element> {
    let text = decode(file, bytes)?;
    let b = text.as_bytes();
    let mut c = Cursor {
        b,
        pos: 0,
        file,
        depth: 0,
    };
    c.skip_ws();
    // Optional BOM.
    if c.starts_with(&[0xEF, 0xBB, 0xBF]) {
        c.pos += 3;
        c.skip_ws();
    }
    // Optional XML declaration / comments before the root.
    loop {
        if c.starts_with(b"<?") {
            while !c.eof() && !c.starts_with(b"?>") {
                c.pos += 1;
            }
            if c.eof() {
                return Err(c.err("`?>`"));
            }
            c.pos += 2;
            c.skip_ws();
        } else if c.starts_with(b"<!--") {
            c.pos += 4;
            while !c.eof() && !c.starts_with(b"-->") {
                c.pos += 1;
            }
            if c.eof() {
                return Err(c.err("`-->`"));
            }
            c.pos += 3;
            c.skip_ws();
        } else {
            break;
        }
    }
    let root = c.element()?;
    c.skip_ws();
    if !c.eof() {
        return Err(c.err("end of file"));
    }
    Ok(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_elements_attrs_text() {
        let src = "<!-- c --><weaponinfo version=\"1\"><weapon type=\"PISTOL\"><damage base=\"10\"/><!-- x --><flags><flag>CAN_AIM</flag></flags></weapon></weaponinfo>";
        let root = parse_xml("w.xml", src.as_bytes()).unwrap();
        assert_eq!(root.name, "weaponinfo");
        assert_eq!(root.attr("version"), Some("1"));
        let w = &root.children[0];
        assert_eq!(w.attr("type"), Some("PISTOL"));
        assert_eq!(w.child("damage").unwrap().attr("base"), Some("10"));
        assert_eq!(
            w.child("flags")
                .unwrap()
                .child("flag")
                .unwrap()
                .text
                .as_deref(),
            Some("CAN_AIM")
        );
    }

    #[test]
    fn mismatched_close_is_error() {
        let e = parse_xml("w.xml", b"<a><b></a>").unwrap_err();
        assert!(matches!(e.kind, ErrorKind::BadHeader { .. }));
    }

    #[test]
    fn unescapes_entities() {
        let root = parse_xml("w.xml", b"<a b=\"x&amp;y\"/>").unwrap();
        assert_eq!(root.attr("b"), Some("x&y"));
    }
}
