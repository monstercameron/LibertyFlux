//! The small XML emitter and renderer descriptions.
//!
//! Shipped files use one tiny shape: an XML declaration, a single root
//! element naming the engine type, and self-closing child elements whose
//! attributes are the named properties. This parser accepts exactly that
//! shape (plus `<name ...></name>` pairs); anything else is an error. It is
//! deliberately not a general XML reader.

use crate::{Error, Result};

/// One child element: a property name plus its attributes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prop {
    /// Element name (for example `vel` or `tintColor`).
    pub name: String,
    /// Attributes in file order.
    pub attrs: Vec<(String, String)>,
}

impl Prop {
    /// The value of attribute `key`, if present.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.attrs
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
}

/// A parsed emitter/renderer file: the engine type plus its properties.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmitterFile {
    /// Root element name (for example `rage__ptxSimpleEmitter`).
    pub root: String,
    /// Child elements in file order.
    pub props: Vec<Prop>,
}

impl EmitterFile {
    /// Find a child element by name, if present.
    #[must_use]
    pub fn prop(&self, name: &str) -> Option<&Prop> {
        self.props.iter().find(|p| p.name == name)
    }
}

/// Parse emitter XML text.
///
/// # Errors
///
/// Returns an error when the text is not well-formed emitter XML.
pub fn parse(text: &str) -> Result<EmitterFile> {
    let bytes = text.as_bytes();
    let mut pos = skip_ws(bytes, 0);
    // Optional XML declaration.
    if bytes.get(pos..pos + 2) == Some(b"<?") {
        let end = find(bytes, pos, b"?>")
            .ok_or_else(|| malformed(text, pos, "unterminated declaration"))?;
        pos = skip_ws(bytes, end + 2);
    }
    // Root open tag: <name> with no attributes in shipped files, but allow
    // attributes for forward compatibility and ignore them.
    if bytes.get(pos) != Some(&b'<') {
        return Err(malformed(text, pos, "expected root element"));
    }
    let root_end =
        find(bytes, pos, b">").ok_or_else(|| malformed(text, pos, "unterminated root tag"))?;
    let root_inner = &text[pos + 1..root_end];
    if root_inner.starts_with('/') || root_inner.ends_with('/') || root_inner.contains('<') {
        return Err(malformed(text, pos, "bad root tag"));
    }
    let root = root_inner
        .split_whitespace()
        .next()
        .ok_or_else(|| malformed(text, pos, "empty root tag"))?;
    check_name(root).map_err(|d| malformed(text, pos, &d))?;
    pos = skip_ws(bytes, root_end + 1);
    // Children.
    let mut props = Vec::new();
    loop {
        if pos >= bytes.len() {
            return Err(malformed(text, pos, "missing root close tag"));
        }
        if bytes.get(pos..pos + 2) == Some(b"</") {
            let end = find(bytes, pos, b">")
                .ok_or_else(|| malformed(text, pos, "unterminated close tag"))?;
            let name = text[pos + 2..end].trim();
            if name != root {
                return Err(malformed(
                    text,
                    pos,
                    &format!("close tag {name:?} does not match root {root:?}"),
                ));
            }
            pos = skip_ws(bytes, end + 1);
            if pos != bytes.len() {
                return Err(malformed(text, pos, "trailing content after root"));
            }
            break;
        }
        if bytes.get(pos) != Some(&b'<') {
            return Err(malformed(text, pos, "expected child element"));
        }
        let (prop, next) = parse_child(text, pos)?;
        props.push(prop);
        pos = skip_ws(bytes, next);
    }
    Ok(EmitterFile {
        root: root.to_string(),
        props,
    })
}

fn parse_child(text: &str, pos: usize) -> Result<(Prop, usize)> {
    let bytes = text.as_bytes();
    let end =
        find(bytes, pos, b">").ok_or_else(|| malformed(text, pos, "unterminated child tag"))?;
    let mut inner = &text[pos + 1..end];
    let self_closing = inner.ends_with('/');
    if self_closing {
        inner = &inner[..inner.len() - 1];
    }
    let mut words = split_attrs(inner).map_err(|d| malformed(text, pos, &d))?;
    if words.is_empty() {
        return Err(malformed(text, pos, "empty child tag"));
    }
    let name = words.remove(0);
    check_name(&name).map_err(|d| malformed(text, pos, &d))?;
    let mut attrs = Vec::with_capacity(words.len());
    for word in words {
        let (key, value) = word
            .split_once('=')
            .ok_or_else(|| malformed(text, pos, &format!("attribute without value: {word:?}")))?;
        check_name(key).map_err(|d| malformed(text, pos, &d))?;
        if value.len() < 2 || !value.starts_with('"') || !value.ends_with('"') {
            return Err(malformed(
                text,
                pos,
                &format!("attribute value must be quoted: {word:?}"),
            ));
        }
        attrs.push((key.to_string(), value[1..value.len() - 1].to_string()));
    }
    let mut next = end + 1;
    if !self_closing {
        // Accept <name ...></name> pairs with nothing between the tags.
        let close = format!("</{name}>");
        if !text[next..].starts_with(&close) {
            return Err(malformed(
                text,
                pos,
                "child element is neither self-closing nor empty",
            ));
        }
        next += close.len();
    }
    Ok((Prop { name, attrs }, next))
}

/// Split a tag's inner text into words, keeping quoted spans together.
fn split_attrs(inner: &str) -> std::result::Result<Vec<String>, String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut in_word = false;
    for ch in inner.chars() {
        if in_quotes {
            current.push(ch);
            if ch == '"' {
                in_quotes = false;
            }
        } else if ch == '"' {
            in_quotes = true;
            in_word = true;
            current.push(ch);
        } else if ch.is_whitespace() {
            if in_word {
                words.push(std::mem::take(&mut current));
                in_word = false;
            }
        } else {
            in_word = true;
            current.push(ch);
        }
    }
    if in_quotes {
        return Err("unterminated quoted value".to_string());
    }
    if in_word {
        words.push(current);
    }
    Ok(words)
}

fn check_name(name: &str) -> std::result::Result<(), String> {
    if name.is_empty()
        || !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == ':')
    {
        return Err(format!("bad element or attribute name: {name:?}"));
    }
    Ok(())
}

fn skip_ws(bytes: &[u8], mut pos: usize) -> usize {
    while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
        pos += 1;
    }
    pos
}

fn find(bytes: &[u8], from: usize, needle: &[u8]) -> Option<usize> {
    bytes[from..]
        .windows(needle.len())
        .position(|w| w == needle)
        .map(|p| p + from)
}

fn malformed(text: &str, pos: usize, detail: &str) -> Error {
    Error::Malformed {
        line: text[..pos.min(text.len())].matches('\n').count() + 1,
        detail: detail.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Hand-written fixture in the shipped shape; no game content.
    const FIXTURE: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\n\
        <rage__ptxSimpleEmitter>\n\
        \t<pos x=\"0.000000\" y=\"1.000000\" z=\"2.000000\"/>\n\
        \t<posRelative value=\"false\"/>\n\
        </rage__ptxSimpleEmitter>\n";

    #[test]
    fn parses_fixture() {
        let file = parse(FIXTURE).unwrap();
        assert_eq!(file.root, "rage__ptxSimpleEmitter");
        assert_eq!(file.props.len(), 2);
        assert_eq!(file.prop("pos").unwrap().get("y"), Some("1.000000"));
        assert_eq!(
            file.prop("posRelative").unwrap().get("value"),
            Some("false")
        );
        assert!(file.prop("missing").is_none());
    }

    #[test]
    fn accepts_empty_pairs_and_rejects_junk() {
        let file = parse("<root><flag value=\"true\"></flag></root>").unwrap();
        assert_eq!(file.props.len(), 1);
        assert!(parse("<root><a/></rooto>").is_err());
        assert!(parse("<root><a></root>").is_err());
        assert!(parse("<root></root>trailing").is_err());
        assert!(parse("not xml").is_err());
        assert!(parse("<root><a v=1/></root>").is_err());
        assert!(parse("").is_err());
    }
}
