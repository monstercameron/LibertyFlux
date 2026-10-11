//! Parser for the plain-text font description files (`fonts.dat` and the
//! Japanese/Russian variants).
//!
//! See the crate documentation for the section layout.

use std::fmt;

/// A parsed font description file.
#[derive(Debug, Clone)]
pub struct FontFile {
    /// Reference resolution from `[RESOLUTION]`.
    pub resolution: (u32, u32),
    /// Advance widths from `[BUTTONS]`, each with its slot comment.
    pub buttons: Vec<ButtonWidth>,
    /// Advance width from `[RADAR_BLIP]`.
    pub radar_blip: u32,
    /// The font definitions in file order.
    pub fonts: Vec<FontDesc>,
}

/// One controller/keyboard glyph advance width plus its slot name.
#[derive(Debug, Clone)]
pub struct ButtonWidth {
    /// Advance width in reference pixels.
    pub width: u32,
    /// Slot comment, e.g. `FO_CONTROLLER_UP`.
    pub slot: String,
}

/// One font definition: glyph map plus metrics.
#[derive(Debug, Clone)]
pub struct FontDesc {
    /// Numeric id from `[FONT_ID]`.
    pub id: u32,
    /// Glyph code per texture slot, from `[MAP]` … `[/MAP]`.
    pub map: Vec<u16>,
    /// Start/end texture slots of the main font range.
    pub main: (u32, u32),
    /// Start/end texture slots of sub-font 1.
    pub sub1: (u32, u32),
    /// Start/end texture slots of sub-font 2.
    pub sub2: (u32, u32),
    /// Start/end texture slots shared across fonts.
    pub common: (u32, u32),
    /// Advance width per texture slot, from `[PROP]` … `[/PROP]`.
    ///
    /// Signed: some fonts carry negative adjustments. The list may be shorter
    /// than [`FontDesc::map`]; slots past its end presumably fall back to
    /// [`FontDesc::unprop`].
    pub prop: Vec<i16>,
    /// Fallback advance width from `[UNPROP]`.
    pub unprop: u32,
    /// Width from `[JAPANESE_SUBFONT_1_WIDTH]` (Japanese font only).
    pub japanese_sub1_width: Option<u32>,
    /// Width from `[JAPANESE_SUBFONT_2_WIDTH]` (Japanese font only).
    pub japanese_sub2_width: Option<u32>,
    /// Spacing values from `[SPACE_BETWEEN_CHARS]`.
    pub spacing: (i32, i32, i32),
    /// Width of the space glyph from `[WHITESPACE]`.
    pub whitespace: u32,
}

/// Error describing why a font file could not be parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FontError {
    /// The input is not valid UTF-8.
    NotUtf8,
    /// A line expected to hold numbers does not.
    BadNumber {
        /// 1-based line number.
        line: usize,
    },
    /// A required section is missing.
    MissingSection {
        /// Section name without brackets.
        section: &'static str,
    },
    /// A block was not closed before end of input.
    UnterminatedBlock {
        /// Section name without brackets.
        section: &'static str,
    },
    /// A value does not fit its field.
    OutOfRange {
        /// 1-based line number.
        line: usize,
    },
}

impl fmt::Display for FontError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotUtf8 => write!(f, "file is not valid UTF-8"),
            Self::BadNumber { line } => write!(f, "bad number on line {line}"),
            Self::MissingSection { section } => {
                write!(f, "missing section [{section}]")
            }
            Self::UnterminatedBlock { section } => {
                write!(f, "block [{section}] is not closed")
            }
            Self::OutOfRange { line } => write!(f, "value out of range on line {line}"),
        }
    }
}

impl std::error::Error for FontError {}

impl FontFile {
    /// Parse a font description file from its bytes.
    ///
    /// Comment lines carry raw non-UTF-8 bytes, so decoding is lossy; only
    /// comments are affected.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(data: &[u8]) -> Result<Self, FontError> {
        let text = String::from_utf8_lossy(data);
        let mut parser = Parser::new(&text);
        parser.run()
    }

    /// Find a font by its numeric id.
    #[must_use]
    pub fn font(&self, id: u32) -> Option<&FontDesc> {
        self.fonts.iter().find(|f| f.id == id)
    }
}

struct Parser<'a> {
    lines: Vec<(usize, &'a str)>,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(text: &'a str) -> Self {
        let lines = text.lines().enumerate().map(|(i, l)| (i + 1, l)).collect();
        Self { lines, pos: 0 }
    }

    // Section dispatch for the whole file; splitting would scatter the grammar.
    #[allow(clippy::too_many_lines)]
    fn run(&mut self) -> Result<FontFile, FontError> {
        let mut resolution = None;
        let mut buttons = Vec::new();
        let mut radar_blip = None;
        let mut fonts = Vec::new();
        let mut current: Option<FontBuilder> = None;
        while let Some((line_no, section)) = self.next_section()? {
            match section {
                "RESOLUTION" => {
                    let values = self.number_line()?;
                    if values.len() != 2 {
                        return Err(FontError::BadNumber { line: line_no });
                    }
                    resolution = Some((values[0], values[1]));
                }
                "BUTTONS" => {
                    buttons = self.button_lines()?;
                }
                "RADAR_BLIP" => {
                    let values = self.number_line()?;
                    if values.len() != 1 {
                        return Err(FontError::BadNumber { line: line_no });
                    }
                    radar_blip = Some(values[0]);
                }
                "FONT_ID" => {
                    if let Some(done) = current.take() {
                        fonts.push(done.build()?);
                    }
                    let values = self.number_line()?;
                    if values.len() != 1 {
                        return Err(FontError::BadNumber { line: line_no });
                    }
                    current = Some(FontBuilder::new(values[0]));
                }
                "MAP" => {
                    let font = current
                        .as_mut()
                        .ok_or(FontError::MissingSection { section: "FONT_ID" })?;
                    let values = self.number_block("MAP")?;
                    let mut map = Vec::with_capacity(values.len());
                    for (value, line_no) in values {
                        map.push(
                            u16::try_from(value)
                                .map_err(|_| FontError::OutOfRange { line: line_no })?,
                        );
                    }
                    font.map = map;
                }
                "MAINFONT" | "SUBFONT_1" | "SUBFONT_2" | "COMMON_FONT" => {
                    let font = current
                        .as_mut()
                        .ok_or(FontError::MissingSection { section: "FONT_ID" })?;
                    let values = self.number_line()?;
                    if values.len() != 2 {
                        return Err(FontError::BadNumber { line: line_no });
                    }
                    let range = (values[0], values[1]);
                    match section {
                        "MAINFONT" => font.main = Some(range),
                        "SUBFONT_1" => font.sub1 = Some(range),
                        "SUBFONT_2" => font.sub2 = Some(range),
                        _ => font.common = Some(range),
                    }
                }
                "PROP" => {
                    let font = current
                        .as_mut()
                        .ok_or(FontError::MissingSection { section: "FONT_ID" })?;
                    // Widths are signed: the Japanese font carries -4.
                    let values = self.number_block("PROP")?;
                    let mut prop = Vec::with_capacity(values.len());
                    for (value, line_no) in values {
                        prop.push(
                            i16::try_from(value)
                                .map_err(|_| FontError::OutOfRange { line: line_no })?,
                        );
                    }
                    font.prop = prop;
                }
                "UNPROP" => {
                    let font = current
                        .as_mut()
                        .ok_or(FontError::MissingSection { section: "FONT_ID" })?;
                    font.unprop = Some(self.single_number()?);
                }
                "JAPANESE_SUBFONT_1_WIDTH" | "JAPANESE_SUBFONT_2_WIDTH" => {
                    let font = current
                        .as_mut()
                        .ok_or(FontError::MissingSection { section: "FONT_ID" })?;
                    let value = self.single_number()?;
                    if section == "JAPANESE_SUBFONT_1_WIDTH" {
                        font.japanese_sub1_width = Some(value);
                    } else {
                        font.japanese_sub2_width = Some(value);
                    }
                }
                "SPACE_BETWEEN_CHARS" => {
                    let font = current
                        .as_mut()
                        .ok_or(FontError::MissingSection { section: "FONT_ID" })?;
                    font.spacing = Some(self.spacing_line()?);
                }
                "WHITESPACE" => {
                    let font = current
                        .as_mut()
                        .ok_or(FontError::MissingSection { section: "FONT_ID" })?;
                    font.whitespace = Some(self.single_number()?);
                }
                _ => {
                    // Unknown section: skip until the next section header.
                    self.skip_section_body();
                }
            }
        }
        if let Some(done) = current.take() {
            fonts.push(done.build()?);
        }
        Ok(FontFile {
            resolution: resolution.ok_or(FontError::MissingSection {
                section: "RESOLUTION",
            })?,
            buttons,
            radar_blip: radar_blip.ok_or(FontError::MissingSection {
                section: "RADAR_BLIP",
            })?,
            fonts,
        })
    }

    /// Advance to the next section header, returning its line and name.
    fn next_section(&mut self) -> Result<Option<(usize, &'a str)>, FontError> {
        while self.pos < self.lines.len() {
            let (line_no, line) = self.lines[self.pos];
            self.pos += 1;
            let body = strip_comment(line).trim();
            if body.is_empty() {
                continue;
            }
            if let Some(name) = section_name(body) {
                if name.starts_with('/') {
                    return Err(FontError::BadNumber { line: line_no });
                }
                return Ok(Some((line_no, name)));
            }
            return Err(FontError::BadNumber { line: line_no });
        }
        Ok(None)
    }

    /// Read the next non-empty data line as unsigned numbers.
    fn number_line(&mut self) -> Result<Vec<u32>, FontError> {
        let (line_no, line) = self.next_data_line()?;
        let body = strip_comment(line).trim();
        split_numbers(body).ok_or(FontError::BadNumber { line: line_no })
    }

    /// Advance to the next non-empty, non-section data line.
    fn next_data_line(&mut self) -> Result<(usize, &'a str), FontError> {
        loop {
            let (line_no, line) =
                self.lines
                    .get(self.pos)
                    .copied()
                    .ok_or(FontError::BadNumber {
                        line: self.last_line(),
                    })?;
            self.pos += 1;
            let body = strip_comment(line).trim();
            if body.is_empty() {
                continue;
            }
            if section_name(body).is_some() {
                return Err(FontError::BadNumber { line: line_no });
            }
            return Ok((line_no, line));
        }
    }

    /// Read one unsigned number from the next data line.
    fn single_number(&mut self) -> Result<u32, FontError> {
        let line_no = self.peek_line();
        let values = self.number_line()?;
        if values.len() != 1 {
            return Err(FontError::BadNumber { line: line_no });
        }
        Ok(values[0])
    }

    /// Read three signed spacing values from the next data line.
    ///
    /// Parsed from the raw line text so that minus signs survive (the
    /// unsigned [`Self::number_line`] helper cannot carry them).
    fn spacing_line(&mut self) -> Result<(i32, i32, i32), FontError> {
        let (line_no, line) = self.next_data_line()?;
        let raw_line = line;
        let body = strip_comment(raw_line).trim().replace(',', " ");
        let mut parts = Vec::new();
        for part in body.split_whitespace() {
            parts.push(
                part.parse::<i32>()
                    .map_err(|_| FontError::BadNumber { line: line_no })?,
            );
        }
        if parts.len() != 3 {
            return Err(FontError::BadNumber { line: line_no });
        }
        Ok((parts[0], parts[1], parts[2]))
    }

    /// Read a `[/NAME]`-terminated block of signed numbers, each with its
    /// 1-based line number for error reports.
    fn number_block(&mut self, name: &'static str) -> Result<Vec<(i32, usize)>, FontError> {
        let mut out = Vec::new();
        let closer = format!("/{name}");
        loop {
            let (line_no, line) = self
                .lines
                .get(self.pos)
                .copied()
                .ok_or(FontError::UnterminatedBlock { section: name })?;
            self.pos += 1;
            let body = strip_comment(line).trim();
            if body.is_empty() {
                continue;
            }
            if let Some(section) = section_name(body) {
                if section == closer.as_str() {
                    return Ok(out);
                }
                return Err(FontError::BadNumber { line: line_no });
            }
            let values = split_signed(body).ok_or(FontError::BadNumber { line: line_no })?;
            for value in values {
                out.push((value, line_no));
            }
        }
    }

    /// Read `[BUTTONS]` lines: a width plus a `#` slot comment.
    fn button_lines(&mut self) -> Result<Vec<ButtonWidth>, FontError> {
        let mut out = Vec::new();
        while let Some((line_no, line)) = self.lines.get(self.pos).copied() {
            let body = strip_comment(line).trim();
            if body.is_empty() {
                self.pos += 1;
                continue;
            }
            if section_name(body).is_some() {
                break;
            }
            self.pos += 1;
            let values = split_numbers(body).ok_or(FontError::BadNumber { line: line_no })?;
            if values.len() != 1 {
                return Err(FontError::BadNumber { line: line_no });
            }
            out.push(ButtonWidth {
                width: values[0],
                slot: comment_text(line).to_owned(),
            });
        }
        Ok(out)
    }

    /// Skip lines until the next section header (unknown sections).
    fn skip_section_body(&mut self) {
        while let Some((_, line)) = self.lines.get(self.pos).copied() {
            let body = strip_comment(line).trim();
            if !body.is_empty() && section_name(body).is_some() {
                break;
            }
            self.pos += 1;
        }
    }

    fn peek_line(&self) -> usize {
        self.lines
            .get(self.pos)
            .map_or_else(|| self.last_line(), |l| l.0)
    }

    fn last_line(&self) -> usize {
        self.lines.last().map_or(0, |l| l.0)
    }
}

#[derive(Default)]
struct FontBuilder {
    id: u32,
    map: Vec<u16>,
    main: Option<(u32, u32)>,
    sub1: Option<(u32, u32)>,
    sub2: Option<(u32, u32)>,
    common: Option<(u32, u32)>,
    prop: Vec<i16>,
    unprop: Option<u32>,
    japanese_sub1_width: Option<u32>,
    japanese_sub2_width: Option<u32>,
    spacing: Option<(i32, i32, i32)>,
    whitespace: Option<u32>,
}

impl FontBuilder {
    fn new(id: u32) -> Self {
        Self {
            id,
            ..Self::default()
        }
    }

    fn build(self) -> Result<FontDesc, FontError> {
        Ok(FontDesc {
            id: self.id,
            map: self.map,
            main: self.main.ok_or(FontError::MissingSection {
                section: "MAINFONT",
            })?,
            sub1: self.sub1.ok_or(FontError::MissingSection {
                section: "SUBFONT_1",
            })?,
            sub2: self.sub2.ok_or(FontError::MissingSection {
                section: "SUBFONT_2",
            })?,
            common: self.common.ok_or(FontError::MissingSection {
                section: "COMMON_FONT",
            })?,
            prop: self.prop,
            unprop: self
                .unprop
                .ok_or(FontError::MissingSection { section: "UNPROP" })?,
            japanese_sub1_width: self.japanese_sub1_width,
            japanese_sub2_width: self.japanese_sub2_width,
            spacing: self.spacing.ok_or(FontError::MissingSection {
                section: "SPACE_BETWEEN_CHARS",
            })?,
            whitespace: self.whitespace.ok_or(FontError::MissingSection {
                section: "WHITESPACE",
            })?,
        })
    }
}

/// Strip `#` and `//` comments from a line.
fn strip_comment(line: &str) -> &str {
    let hash = line.find('#').unwrap_or(line.len());
    let slash = line.find("//").unwrap_or(line.len());
    line[..hash.min(slash)].trim_end()
}

/// Text of the `#` comment on a line, without markers.
fn comment_text(line: &str) -> &str {
    match line.find('#') {
        Some(pos) => line[pos + 1..].split("//").next().unwrap_or("").trim(),
        None => "",
    }
}

/// Name inside a `[NAME]` header, if the line is one.
fn section_name(body: &str) -> Option<&str> {
    let inner = body.strip_prefix('[')?.strip_suffix(']')?;
    if inner.is_empty() || inner.contains(['[', ']']) {
        return None;
    }
    Some(inner)
}

/// Parse comma- or space-separated unsigned numbers.
fn split_numbers(body: &str) -> Option<Vec<u32>> {
    let mut out = Vec::new();
    for part in body.replace(',', " ").split_whitespace() {
        out.push(part.parse::<u32>().ok()?);
    }
    if out.is_empty() {
        return None;
    }
    Some(out)
}

/// Parse comma- or space-separated signed numbers.
fn split_signed(body: &str) -> Option<Vec<i32>> {
    let mut out = Vec::new();
    for part in body.replace(',', " ").split_whitespace() {
        out.push(part.parse::<i32>().ok()?);
    }
    if out.is_empty() {
        return None;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "\
# comment
[RESOLUTION]
512,488
[BUTTONS]
22 # FO_CONTROLLER_UP
29 # FO_CONTROLLER_BUTTON_A // suffix
[RADAR_BLIP]
29 # SIZE
[FONT_ID]
0
[MAP]
33 34 35
[/MAP]
[MAINFONT]
0 2
[SUBFONT_1]
2 3
[SUBFONT_2]
0 0
[COMMON_FONT]
3 3
[PROP]
26 23 15
[/PROP]
[UNPROP]
26
[SPACE_BETWEEN_CHARS]
0 -2 0
[WHITESPACE]
8
";

    #[test]
    fn parses_fixture() {
        let file = FontFile::parse(FIXTURE.as_bytes()).unwrap();
        assert_eq!(file.resolution, (512, 488));
        assert_eq!(file.buttons.len(), 2);
        assert_eq!(file.buttons[0].width, 22);
        assert_eq!(file.buttons[0].slot, "FO_CONTROLLER_UP");
        assert_eq!(file.radar_blip, 29);
        assert_eq!(file.fonts.len(), 1);
        let font = &file.fonts[0];
        assert_eq!(font.id, 0);
        assert_eq!(font.map, vec![33, 34, 35]);
        assert_eq!(font.main, (0, 2));
        assert_eq!(font.prop, vec![26, 23, 15]);
        assert_eq!(font.unprop, 26);
        assert_eq!(font.spacing, (0, -2, 0));
        assert_eq!(font.whitespace, 8);
        assert!(file.font(0).is_some());
        assert!(file.font(9).is_none());
    }

    #[test]
    fn rejects_missing_section() {
        let err = FontFile::parse(b"[RESOLUTION]\n1,2\n").unwrap_err();
        assert!(matches!(err, FontError::MissingSection { .. }));
    }

    #[test]
    fn rejects_unterminated_block() {
        let bad = "[RESOLUTION]\n1,2\n[RADAR_BLIP]\n1\n[FONT_ID]\n0\n[MAP]\n1 2\n";
        let err = FontFile::parse(bad.as_bytes()).unwrap_err();
        assert!(matches!(err, FontError::UnterminatedBlock { .. }));
    }
}
