//! Minimal reader for `frontend_menus.xml`: menu definitions whose options
//! reference GXT text labels.
//!
//! The schema is small and regular — a root element holding section elements
//! (`sMenuDisplayValue`, `sMenuScreen`), each holding menu elements (`menu`,
//! `menupc`, `menu360`, `menups3`, `menupldebug`), each holding self-closing
//! option elements (`options`, `options360`, `optionsps3`, `optionspc`) — so
//! this module implements a tiny tag scanner for exactly that shape instead
//! of a general XML parser. Comments are skipped; character entities are left
//! as written (the shipped files use none in attribute values).

use std::fmt;

/// A parsed `frontend_menus.xml` file.
#[derive(Debug, Clone)]
pub struct MenuFile {
    /// Value of the root `version` attribute.
    pub version: String,
    /// Sections in file order.
    pub sections: Vec<MenuSection>,
}

/// One section of menus (`sMenuDisplayValue`, `sMenuScreen`).
#[derive(Debug, Clone)]
pub struct MenuSection {
    /// Element name of the section.
    pub name: String,
    /// Menus in file order.
    pub menus: Vec<Menu>,
}

/// One menu: an enum id plus its selectable options.
#[derive(Debug, Clone)]
pub struct Menu {
    /// Element name (`menu`, `menupc`, …).
    pub element: String,
    /// Value of the `enum` attribute, when present.
    pub enum_name: Option<String>,
    /// Value of the `HeaderText` attribute, when present.
    pub header_text: Option<String>,
    /// Options in file order.
    pub options: Vec<MenuOption>,
}

/// One menu option row.
///
/// Display menus carry `text` (a GXT label) plus `action` and `value`; screen
/// entries carry `label`, `scaler` and `display_value` instead. All fields
/// are optional because the file mixes both shapes.
#[derive(Debug, Clone)]
pub struct MenuOption {
    /// Element name (`options`, `optionspc`, …).
    pub element: String,
    /// GXT text label from the `text` attribute, when present.
    pub text: Option<String>,
    /// Action name, when present.
    pub action: Option<String>,
    /// Value word, when present.
    pub value: Option<String>,
    /// Screen-entry label, when present.
    pub label: Option<String>,
    /// Screen-entry scaler, when present.
    pub scaler: Option<String>,
    /// Screen-entry display value, when present.
    pub display_value: Option<String>,
}

/// Error describing why a menu file could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuError {
    /// The input is not valid UTF-8.
    NotUtf8,
    /// A tag is malformed.
    BadTag {
        /// Byte offset of the tag.
        offset: usize,
    },
    /// Tags nest in an unexpected way.
    BadNesting {
        /// Byte offset of the tag.
        offset: usize,
    },
    /// The root element is missing or has no version.
    BadRoot,
}

impl fmt::Display for MenuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotUtf8 => write!(f, "file is not valid UTF-8"),
            Self::BadTag { offset } => write!(f, "malformed tag at byte {offset}"),
            Self::BadNesting { offset } => {
                write!(f, "unexpected nesting at byte {offset}")
            }
            Self::BadRoot => write!(f, "missing root element or version"),
        }
    }
}

impl std::error::Error for MenuError {}

impl MenuFile {
    /// Parse a `frontend_menus.xml` file from its bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    // Section dispatch for the whole file; splitting would scatter the grammar.
    #[allow(clippy::too_many_lines)]
    pub fn parse(data: &[u8]) -> Result<Self, MenuError> {
        let text = std::str::from_utf8(data).map_err(|_| MenuError::NotUtf8)?;
        let tags = scan_tags(text)?;
        let mut sections = Vec::new();
        let mut current_section: Option<MenuSection> = None;
        let mut current_menu: Option<Menu> = None;
        let mut root_version: Option<String> = None;
        let mut depth = 0usize;
        for tag in &tags {
            match tag.kind {
                TagKind::Open => {
                    depth += 1;
                    match depth {
                        1 => {
                            if tag.name != "FrontendMenu" {
                                return Err(MenuError::BadRoot);
                            }
                            root_version = attr(tag, "version");
                        }
                        2 => {
                            if current_section.is_some() {
                                return Err(MenuError::BadNesting { offset: tag.offset });
                            }
                            current_section = Some(MenuSection {
                                name: tag.name.clone(),
                                menus: Vec::new(),
                            });
                        }
                        3 => {
                            if current_menu.is_some()
                                || current_section.is_none()
                                || !is_menu_element(&tag.name)
                            {
                                return Err(MenuError::BadNesting { offset: tag.offset });
                            }
                            current_menu = Some(Menu {
                                element: tag.name.clone(),
                                enum_name: attr(tag, "enum"),
                                header_text: attr(tag, "HeaderText"),
                                options: Vec::new(),
                            });
                        }
                        _ => {
                            return Err(MenuError::BadNesting { offset: tag.offset });
                        }
                    }
                }
                TagKind::Empty => {
                    if depth != 3 || !is_option_element(&tag.name) {
                        return Err(MenuError::BadNesting { offset: tag.offset });
                    }
                    let menu = current_menu
                        .as_mut()
                        .ok_or(MenuError::BadNesting { offset: tag.offset })?;
                    menu.options.push(MenuOption {
                        element: tag.name.clone(),
                        text: attr(tag, "text"),
                        action: attr(tag, "action"),
                        value: attr(tag, "value"),
                        label: attr(tag, "label"),
                        scaler: attr(tag, "scaler"),
                        display_value: attr(tag, "displayValue"),
                    });
                }
                TagKind::Close => {
                    match depth {
                        3 => {
                            let menu = current_menu
                                .take()
                                .ok_or(MenuError::BadNesting { offset: tag.offset })?;
                            if menu.element != tag.name {
                                return Err(MenuError::BadNesting { offset: tag.offset });
                            }
                            current_section
                                .as_mut()
                                .ok_or(MenuError::BadNesting { offset: tag.offset })?
                                .menus
                                .push(menu);
                        }
                        2 => {
                            let section = current_section
                                .take()
                                .ok_or(MenuError::BadNesting { offset: tag.offset })?;
                            if section.name != tag.name {
                                return Err(MenuError::BadNesting { offset: tag.offset });
                            }
                            sections.push(section);
                        }
                        1 => {
                            if tag.name != "FrontendMenu" {
                                return Err(MenuError::BadNesting { offset: tag.offset });
                            }
                        }
                        _ => {
                            return Err(MenuError::BadNesting { offset: tag.offset });
                        }
                    }
                    depth = depth.saturating_sub(1);
                }
            }
        }
        if depth != 0 {
            return Err(MenuError::BadNesting { offset: text.len() });
        }
        Ok(Self {
            version: root_version.ok_or(MenuError::BadRoot)?,
            sections,
        })
    }

    /// Iterate over every option in every menu.
    pub fn iter_options(&self) -> impl Iterator<Item = (&Menu, &MenuOption)> {
        self.sections
            .iter()
            .flat_map(|s| s.menus.iter())
            .flat_map(|m| m.options.iter().map(move |o| (m, o)))
    }

    /// Iterate over every GXT label referenced by `text` attributes.
    pub fn iter_labels(&self) -> impl Iterator<Item = &str> {
        self.iter_options().filter_map(|(_, o)| o.text.as_deref())
    }
}

fn is_menu_element(name: &str) -> bool {
    matches!(
        name,
        "menu" | "menupc" | "menu360" | "menups3" | "menupldebug"
    )
}

fn is_option_element(name: &str) -> bool {
    matches!(name, "options" | "options360" | "optionsps3" | "optionspc")
}

fn attr(tag: &Tag, name: &str) -> Option<String> {
    tag.attrs
        .iter()
        .find(|(k, _)| k == name)
        .map(|(_, v)| v.clone())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TagKind {
    Open,
    Empty,
    Close,
}

#[derive(Debug, Clone)]
struct Tag {
    kind: TagKind,
    name: String,
    attrs: Vec<(String, String)>,
    offset: usize,
}

/// Scan `<...>` tags, skipping comments and declarations.
fn scan_tags(text: &str) -> Result<Vec<Tag>, MenuError> {
    let mut tags = Vec::new();
    let mut pos = 0;
    while let Some(start_rel) = text[pos..].find('<') {
        let start = pos + start_rel;
        if text[start..].starts_with("<!--") {
            let end = text[start..]
                .find("-->")
                .ok_or(MenuError::BadTag { offset: start })?;
            pos = start + end + 3;
            continue;
        }
        let end_rel = text[start..]
            .find('>')
            .ok_or(MenuError::BadTag { offset: start })?;
        let end = start + end_rel;
        let mut inner = &text[start + 1..end];
        // Skip declarations such as <?xml ...?>.
        if inner.starts_with('?') || inner.starts_with('!') {
            pos = end + 1;
            continue;
        }
        let kind;
        if let Some(rest) = inner.strip_prefix('/') {
            kind = TagKind::Close;
            inner = rest;
        } else if inner.trim_end().ends_with('/') {
            kind = TagKind::Empty;
            inner = inner.trim_end();
            inner = inner[..inner.len() - 1].trim_end();
        } else {
            kind = TagKind::Open;
        }
        let (name, attrs) = split_tag(inner).ok_or(MenuError::BadTag { offset: start })?;
        if name.is_empty() {
            return Err(MenuError::BadTag { offset: start });
        }
        tags.push(Tag {
            kind,
            name,
            attrs,
            offset: start,
        });
        pos = end + 1;
    }
    Ok(tags)
}

/// Split a tag body into its name and `key="value"` attributes.
fn split_tag(inner: &str) -> Option<(String, Vec<(String, String)>)> {
    let mut chars = inner.chars().peekable();
    let mut name = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            break;
        }
        name.push(ch);
        chars.next();
    }
    let mut attrs = Vec::new();
    loop {
        while chars.peek().is_some_and(|ch| ch.is_whitespace()) {
            chars.next();
        }
        if chars.peek().is_none() {
            break;
        }
        let mut key = String::new();
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() || ch == '=' {
                break;
            }
            key.push(ch);
            chars.next();
        }
        while chars.peek().is_some_and(|ch| ch.is_whitespace()) {
            chars.next();
        }
        if chars.next() != Some('=') {
            return None;
        }
        while chars.peek().is_some_and(|ch| ch.is_whitespace()) {
            chars.next();
        }
        if chars.next() != Some('"') {
            return None;
        }
        let mut value = String::new();
        loop {
            match chars.next()? {
                '"' => break,
                ch => value.push(ch),
            }
        }
        attrs.push((key, value));
    }
    Some((name, attrs))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "\
<FrontendMenu version=\"1\">
<!-- a comment -->
<sMenuDisplayValue>
<menu enum=\"MENU_DISPLAY_ON_OFF\">
<options text=\"MO_OFF\" action=\"ACTION_NONE\" value=\"0\"/>
<options text=\"MO_ON\" action=\"ACTION_NONE\" value=\"0\"/>
</menu>
<menupc enum=\"MENU_PC_ONLY\" HeaderText=\"HDR\">
<optionspc label=\"L1\" scaler=\"S1\" displayValue=\"D1\" action=\"A\" value=\"2\"/>
</menupc>
</sMenuDisplayValue>
</FrontendMenu>
";

    #[test]
    fn parses_fixture() {
        let file = MenuFile::parse(FIXTURE.as_bytes()).unwrap();
        assert_eq!(file.version, "1");
        assert_eq!(file.sections.len(), 1);
        assert_eq!(file.sections[0].name, "sMenuDisplayValue");
        assert_eq!(file.sections[0].menus.len(), 2);
        let menu = &file.sections[0].menus[0];
        assert_eq!(menu.enum_name.as_deref(), Some("MENU_DISPLAY_ON_OFF"));
        assert_eq!(menu.options.len(), 2);
        assert_eq!(menu.options[0].text.as_deref(), Some("MO_OFF"));
        let labels: Vec<_> = file.iter_labels().collect();
        assert_eq!(labels, vec!["MO_OFF", "MO_ON"]);
    }

    #[test]
    fn rejects_mismatched_close() {
        let bad = "<FrontendMenu version=\"1\"><sMenuDisplayValue></FrontendMenu>";
        let err = MenuFile::parse(bad.as_bytes()).unwrap_err();
        assert!(matches!(
            err,
            MenuError::BadNesting { .. } | MenuError::BadRoot
        ));
    }
}
