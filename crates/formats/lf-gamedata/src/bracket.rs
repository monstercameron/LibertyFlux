//! Bracket-section tables: HUD, frontend, fonts, radio, visual settings.
//!
//! Several UI and tuning files share one shape: `#` comments, `[NAME]`
//! section headers (`HD`/`CRT` display targets, font tables, radio prefs),
//! and whitespace- or comma-separated value rows. This module parses that
//! shared shape once:
//!
//! * `hud.dat`: item name, `x,y` position, `w,h` size, colour name, alpha.
//! * `hudColor.dat`: colour name plus R G B.
//! * `frontend_*.dat`, `RadioLogo.dat`: pref name plus two floats.
//! * `fonts.dat`: `[RESOLUTION]`, `[BUTTONS]`, per-font sections of widths.
//! * `visualSettings.dat`: no brackets — plain `dotted.name value...` rows
//!   (see [`parse_visual_settings`]).
//!
//! Rows are kept as name-plus-tokens; [`HudColor`] and [`HudItem`] type
//! the two HUD tables fully.

use crate::text::{CommentStyle, logical_lines, parse_f32, split_ws};
use crate::{Error, ErrorKind, Result, decode};

/// One row: name plus raw value tokens, with its line number.
#[derive(Debug, Clone)]
pub struct ValueRow {
    /// 1-based line number.
    pub line: usize,
    /// Row name (first token).
    pub name: String,
    /// Remaining tokens.
    pub values: Vec<String>,
}

/// One bracket section: name plus rows.
#[derive(Debug, Clone)]
pub struct BracketSection {
    /// Section name without brackets.
    pub name: String,
    /// Rows in file order.
    pub rows: Vec<ValueRow>,
}

/// Parse any `[SECTION]` file from bytes. Rows before the first header
/// are an error.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_bracket(file: &str, bytes: &[u8]) -> Result<Vec<BracketSection>> {
    let text = decode(file, bytes)?;
    let mut out: Vec<BracketSection> = Vec::new();
    for l in logical_lines(&text, CommentStyle::HASH, false) {
        let code = l.code.trim();
        if let Some(name) = code.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            out.push(BracketSection {
                name: name.trim().to_string(),
                rows: Vec::new(),
            });
            continue;
        }
        let section = out.last_mut().ok_or_else(|| {
            Error::new(
                file,
                l.num,
                ErrorKind::UnknownMarker {
                    marker: "row before first section".to_string(),
                },
            )
        })?;
        let f = split_ws(code);
        if f.is_empty() {
            continue;
        }
        section.rows.push(ValueRow {
            line: l.num,
            name: f[0].clone(),
            values: f[1..].to_vec(),
        });
    }
    Ok(out)
}

/// One `hudColor.dat` row: colour name plus RGB.
#[derive(Debug, Clone)]
pub struct HudColor {
    /// Colour name.
    pub name: String,
    /// Red, green, blue.
    pub rgb: [u8; 3],
}

/// Type the rows of one `hudColor.dat` section.
///
/// Trailing parenthesised notes (`(MISSION WAYPOINT)`) without a `#`
/// marker are cut off first; the episode files use them.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn hud_colors(file: &str, section: &BracketSection) -> Result<Vec<HudColor>> {
    let mut out = Vec::new();
    for r in &section.rows {
        let values: Vec<&String> = r
            .values
            .iter()
            .take_while(|v| !v.starts_with('('))
            .collect();
        if values.len() != 3 {
            return Err(Error::new(
                file,
                r.line,
                ErrorKind::FieldCount {
                    expected: Some(4),
                    found: r.values.len() + 1,
                },
            ));
        }
        let rgb = [
            values[0].parse::<u8>(),
            values[1].parse::<u8>(),
            values[2].parse::<u8>(),
        ];
        let mut c = [0u8; 3];
        for (i, v) in rgb.into_iter().enumerate() {
            c[i] = v.map_err(|_| {
                Error::new(
                    file,
                    r.line,
                    ErrorKind::BadScalar {
                        field: i + 1,
                        want: "u8",
                    },
                )
            })?;
        }
        out.push(HudColor {
            name: r.name.clone(),
            rgb: c,
        });
    }
    Ok(out)
}

/// One `hud.dat` row: item placement.
#[derive(Debug, Clone)]
pub struct HudItem {
    /// Item name.
    pub name: String,
    /// Position (x, y).
    pub pos: [f32; 2],
    /// Size (w, h).
    pub size: [f32; 2],
    /// Colour name.
    pub colour: String,
    /// Alpha 0-255.
    pub alpha: u8,
}

fn pair(file: &str, line: usize, field: usize, tok: &str) -> Result<[f32; 2]> {
    let parts: Vec<&str> = tok.split(',').collect();
    if parts.len() != 2 {
        return Err(Error::new(
            file,
            line,
            ErrorKind::FieldCount {
                expected: Some(2),
                found: parts.len(),
            },
        ));
    }
    let bad = |_| Error::new(file, line, ErrorKind::BadScalar { field, want: "f32" });
    Ok([
        parts[0].parse::<f32>().map_err(bad)?,
        parts[1].parse::<f32>().map_err(bad)?,
    ])
}

/// Type the rows of one `hud.dat` section.
///
/// Three shipped rows write the size pair with a space after the comma
/// (`0.0,  0.15`), which whitespace splitting turns into two tokens; a
/// token ending in `,` is glued back to its neighbour first.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn hud_items(file: &str, section: &BracketSection) -> Result<Vec<HudItem>> {
    let mut out = Vec::new();
    for r in &section.rows {
        let mut values: Vec<String> = Vec::with_capacity(r.values.len());
        for tok in &r.values {
            let glue = values.last().is_some_and(|prev| prev.ends_with(','));
            if glue {
                if let Some(prev) = values.last_mut() {
                    prev.push_str(tok);
                }
            } else {
                values.push(tok.clone());
            }
        }
        if values.len() != 4 {
            return Err(Error::new(
                file,
                r.line,
                ErrorKind::FieldCount {
                    expected: Some(5),
                    found: r.values.len() + 1,
                },
            ));
        }
        out.push(HudItem {
            name: r.name.clone(),
            pos: pair(file, r.line, 1, &values[0])?,
            size: pair(file, r.line, 2, &values[1])?,
            colour: values[2].clone(),
            alpha: values[3].parse::<u8>().map_err(|_| {
                Error::new(
                    file,
                    r.line,
                    ErrorKind::BadScalar {
                        field: 4,
                        want: "u8",
                    },
                )
            })?,
        });
    }
    Ok(out)
}

/// One `visualSettings.dat` row: dotted name plus float values.
#[derive(Debug, Clone)]
pub struct VisualSetting {
    /// Setting name (`heightReflect.width`, ...).
    pub name: String,
    /// Values.
    pub values: Vec<f32>,
}

/// Parse a `visualSettings.dat`-shaped file: `name value...` rows.
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn parse_visual_settings(file: &str, bytes: &[u8]) -> Result<Vec<VisualSetting>> {
    let text = decode(file, bytes)?;
    let mut out = Vec::new();
    for l in logical_lines(&text, CommentStyle::HASH, false) {
        let f = split_ws(&l.code);
        if f.is_empty() {
            continue;
        }
        let mut values = Vec::with_capacity(f.len() - 1);
        for i in 1..f.len() {
            values.push(parse_f32(file, l.num, &f, i)?);
        }
        out.push(VisualSetting {
            name: f[0].clone(),
            values,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bracket_sections() {
        let src =
            "# c\r\n[HD]\r\nHUD_COLOUR_RED 153 69 69\r\n[CRT]\r\nHUD_COLOUR_RED 150 70 70\r\n";
        let s = parse_bracket("hudColor.dat", src.as_bytes()).unwrap();
        assert_eq!(s.len(), 2);
        let colors = hud_colors("hudColor.dat", &s[0]).unwrap();
        assert_eq!(colors[0].rgb, [153, 69, 69]);
    }

    #[test]
    fn parses_hud_item() {
        let src = "[HD]\r\nHUD_BIG_MESSAGE_COMPLETE 0.5,0.45 0.631,1.0 HUD_COLOUR_RED 255\r\n";
        let s = parse_bracket("hud.dat", src.as_bytes()).unwrap();
        let items = hud_items("hud.dat", &s[0]).unwrap();
        assert_eq!(items[0].pos, [0.5, 0.45]);
        assert_eq!(items[0].alpha, 255);
    }

    #[test]
    fn parses_visual_settings() {
        let src = "rain.gravity.z -0.98\r\nrain.NumberParticles 16384\r\n";
        let v = parse_visual_settings("visualSettings.dat", src.as_bytes()).unwrap();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].values, vec![-0.98]);
    }
}
