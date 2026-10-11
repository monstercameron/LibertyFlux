//! Parsers for `hud.dat` (HUD item layout) and `hudColor.dat` (HUD palette).
//!
//! Both files share the sectioned shape: one section per display mode
//! (`[HD]`, `[CRT]`) with one row per item or colour.

use crate::dat::{DataError, data_file_lines, section_name};

/// A parsed `hud.dat` file.
#[derive(Debug, Clone)]
pub struct HudFile {
    /// Sections in file order (display modes).
    pub sections: Vec<HudSection>,
}

/// One display-mode section of HUD items.
#[derive(Debug, Clone)]
pub struct HudSection {
    /// Section name, e.g. `HD`.
    pub name: String,
    /// Items in file order.
    pub items: Vec<HudItem>,
}

/// One HUD item: name, position, size, colour reference and alpha.
///
/// Positions and sizes are pairs written as `x,y` in the file. The colour is
/// a name referencing `hudColor.dat`; a few rows omit the colour and alpha.
#[derive(Debug, Clone)]
pub struct HudItem {
    /// Item name, e.g. `HUD_RADAR`.
    pub name: String,
    /// Position pair from the file.
    pub position: (f32, f32),
    /// Size pair from the file.
    pub size: (f32, f32),
    /// Colour name, when the row carries one.
    pub colour: Option<String>,
    /// Alpha value, when the row carries one.
    pub alpha: Option<u32>,
}

impl HudFile {
    /// Parse a `hud.dat` file from its bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(data: &[u8]) -> Result<Self, DataError> {
        let mut sections = Vec::new();
        let mut current: Option<HudSection> = None;
        for item in data_file_lines(data)? {
            if let Some(name) = section_name(&item.text) {
                if let Some(done) = current.take() {
                    sections.push(done);
                }
                current = Some(HudSection {
                    name: name.to_owned(),
                    items: Vec::new(),
                });
                continue;
            }
            let section = current
                .as_mut()
                .ok_or(DataError::BadLine { line: item.line })?;
            section
                .items
                .push(parse_item(&item.text).ok_or(DataError::BadLine { line: item.line })?);
        }
        if let Some(done) = current.take() {
            sections.push(done);
        }
        Ok(Self { sections })
    }

    /// Find a section by name.
    #[must_use]
    pub fn section(&self, name: &str) -> Option<&HudSection> {
        self.sections.iter().find(|s| s.name == name)
    }
}

fn parse_pair(text: &str) -> Option<(f32, f32)> {
    let (a, b) = text.split_once(',')?;
    Some((a.trim().parse().ok()?, b.trim().parse().ok()?))
}

fn parse_item(text: &str) -> Option<HudItem> {
    // Pairs are usually written `x,y` but some rows pad a space after the
    // comma (`0.0,  0.15`); normalise before splitting.
    let mut normalised = text.replace(" ,", ",");
    while normalised.contains(", ") {
        normalised = normalised.replace(", ", ",");
    }
    let mut parts = normalised.split_whitespace();
    let name = parts.next()?.to_owned();
    let position = parse_pair(parts.next()?)?;
    let size = parse_pair(parts.next()?)?;
    let colour = parts.next().map(str::to_owned);
    let alpha = parts.next().map(str::parse::<u32>).transpose().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some(HudItem {
        name,
        position,
        size,
        colour,
        alpha,
    })
}

/// An RGB triple from `hudColor.dat`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    /// Red channel.
    pub r: u8,
    /// Green channel.
    pub g: u8,
    /// Blue channel.
    pub b: u8,
}

/// A parsed `hudColor.dat` file.
#[derive(Debug, Clone)]
pub struct HudColours {
    /// Sections in file order (display modes).
    pub sections: Vec<HudColourSection>,
}

/// One display-mode section of the palette.
#[derive(Debug, Clone)]
pub struct HudColourSection {
    /// Section name, e.g. `HD`.
    pub name: String,
    /// Colours in file order.
    pub colours: Vec<HudColour>,
}

/// One named palette colour.
#[derive(Debug, Clone)]
pub struct HudColour {
    /// Colour name, e.g. `HUD_COLOUR_RED`.
    pub name: String,
    /// RGB triple.
    pub rgb: Rgb,
}

impl HudColours {
    /// Parse a `hudColor.dat` file from its bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(data: &[u8]) -> Result<Self, DataError> {
        let mut sections = Vec::new();
        let mut current: Option<HudColourSection> = None;
        for item in data_file_lines(data)? {
            if let Some(name) = section_name(&item.text) {
                if let Some(done) = current.take() {
                    sections.push(done);
                }
                current = Some(HudColourSection {
                    name: name.to_owned(),
                    colours: Vec::new(),
                });
                continue;
            }
            let section = current
                .as_mut()
                .ok_or(DataError::BadLine { line: item.line })?;
            // Some rows carry a trailing parenthesised note; it is a comment.
            let row = item.text.split('(').next().unwrap_or("").trim();
            let mut parts = row.split_whitespace();
            let name = parts.next().ok_or(DataError::BadLine { line: item.line })?;
            let mut rgb = Vec::new();
            for part in parts {
                rgb.push(
                    part.parse::<u8>()
                        .map_err(|_| DataError::BadLine { line: item.line })?,
                );
            }
            if rgb.len() != 3 {
                return Err(DataError::BadLine { line: item.line });
            }
            section.colours.push(HudColour {
                name: name.to_owned(),
                rgb: Rgb {
                    r: rgb[0],
                    g: rgb[1],
                    b: rgb[2],
                },
            });
        }
        if let Some(done) = current.take() {
            sections.push(done);
        }
        Ok(Self { sections })
    }

    /// Find a section by name.
    #[must_use]
    pub fn section(&self, name: &str) -> Option<&HudColourSection> {
        self.sections.iter().find(|s| s.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hud_items() {
        let input = b"[HD]\nHUD_RADAR 0.064,0.745 0.161,0.211 HUD_COLOUR_WHITE 150\nHUD_ICON 0.0,0.0 0.046,0.060\n";
        let file = HudFile::parse(input).unwrap();
        let section = file.section("HD").unwrap();
        assert_eq!(section.items.len(), 2);
        assert_eq!(section.items[0].name, "HUD_RADAR");
        assert_eq!(section.items[0].alpha, Some(150));
        assert_eq!(section.items[1].colour, None);
        assert_eq!(section.items[1].alpha, None);
    }

    #[test]
    fn parses_colours() {
        let input = b"[HD]\nHUD_COLOUR_RED 153 69 69\n";
        let file = HudColours::parse(input).unwrap();
        let colour = &file.section("HD").unwrap().colours[0];
        assert_eq!(colour.name, "HUD_COLOUR_RED");
        assert_eq!(
            colour.rgb,
            Rgb {
                r: 153,
                g: 69,
                b: 69
            }
        );
    }

    #[test]
    fn rejects_short_colour_row() {
        let err = HudColours::parse(b"[HD]\nHUD_COLOUR_RED 153 69\n").unwrap_err();
        assert!(matches!(err, DataError::BadLine { .. }));
    }
}
