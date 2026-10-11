//! Reader for the game's plain-text gameplay data tables.
//!
//! The game ships hundreds of loose text files that tune vehicles, peds,
//! weapons, weather, population and the user interface. They live in the
//! `common/data` folder (plus per-episode overrides and a few files under
//! `pc/data`). This crate parses them into typed Rust structs.
//!
//! # File families
//!
//! Every file here is line oriented, US-ASCII, CRLF line endings, parsed
//! from bytes (never from memory-mapped structs). The families differ in
//! comment syntax, field separator and record framing:
//!
//! | Family | Comment | Separator | Framing | Examples |
//! |---|---|---|---|
//! | Semicolon tables | `;` to end of line, `#` for disabled rows | whitespace | one row per line | `handling.dat`, `object.dat` (commas), `numplate.dat` |
//! | Hash sections | `#` to end of line (also trailing) | commas | `name ... end` blocks | `*.ide`, `carcols.dat`, `carmods.dat` |
//! | Slash tables | `//` to end of line | whitespace or commas | one row per line | `popcycle.dat`, `timecyc*.dat`, `*.csv` |
//! | Keyword rules | `#` or `//` | whitespace | verb-led lines | `clothes.dat`, `furnitur.dat`, `vehOff.csv` |
//! | Directives | `#` | whitespace | `KEY args` lines | `gta.dat`, `default.dat`, `images.txt` |
//! | Bracket sections | `#` | whitespace/commas | `[NAME]` groups | `hud.dat`, `hudColor.dat`, `fonts.dat`, `frontend_*.dat` |
//! | XML | `<!-- -->` | markup | elements | `WeaponInfo.xml`, `frontend_menus.xml` |
//!
//! # What is covered
//!
//! Typed parsers exist for the load-critical tables: vehicle handling
//! ([`handling`]), item definitions ([`ide`]), car colours ([`carcols`]),
//! population groups ([`groups`]), ped personality and variations
//! ([`ped_personality`], [`ped_variations`]), weapons ([`weapon_info`]),
//! time cycle ([`timecyc`]), water ([`water`]), load lists ([`load_list`]),
//! population cycle ([`popcycle`]), object physics ([`object`]), materials
//! ([`materials`]), HUD and frontend tables ([`bracket`]), relationship
//! tables ([`relations`]), effects tables ([`effects`]) and miscellaneous
//! CSV and keyword files ([`misc`]).
//!
//! # What is not text
//!
//! A few `common/data` files with text-looking names are binary grids and
//! are out of scope for this crate: `pedpopulation.dat`,
//! `precincts.dat`, `WorldBlanket.dat`, `polydensity.dat` and
//! `navprecalc.dat` (magic `NAVP`). They are detected by the integration
//! test and reported as binary, not parsed.
//!
//! # Error handling
//!
//! Every parser returns [`Result`] with an [`Error`] that carries the file
//! name, the 1-based line number and a machine-readable [`ErrorKind`].
//! Parsers never panic on malformed input.
//!
//! # Sources
//!
//! Field meanings come first from the header comments inside the game's
//! own data files (which document units, ranges and flag bits), second
//! from the observed values across all shipped files, and third from
//! public documentation (`GTAMods` wiki per-file pages for handling,
//! carcols, IDE sections and time cycle, read for layout only). No parser
//! code was copied from any existing tool.

pub mod bracket;
pub mod carcols;
pub mod effects;
pub mod groups;
pub mod handling;
pub mod ide;
pub mod load_list;
pub mod materials;
pub mod misc;
pub mod object;
pub mod ped_personality;
pub mod ped_variations;
pub mod popcycle;
pub mod relations;
pub mod route;
pub mod scan;
pub mod text;
pub mod timecyc;
pub mod water;
pub mod weapon_info;
pub mod xml;

use std::fmt;

/// Machine-readable parse failure categories.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    /// A line has fewer or more fields than the record needs.
    FieldCount {
        /// How many fields the record needs, or `None` for variable width.
        expected: Option<usize>,
        /// How many fields the line had.
        found: usize,
    },
    /// A field does not parse as the expected scalar.
    BadScalar {
        /// Field index within the record (0-based).
        field: usize,
        /// Which scalar type was expected (`"f32"`, `"u32"`, ...).
        want: &'static str,
    },
    /// A section or block was not closed with `end`.
    Unterminated {
        /// Section name that was left open.
        section: String,
    },
    /// An `end` line with no open section, or a row outside any section.
    StrayEnd,
    /// A record marker or directive keyword is not recognised.
    UnknownMarker {
        /// The offending token.
        marker: String,
    },
    /// Expected a header, version or magic line and found something else.
    BadHeader {
        /// What was expected, in words.
        want: &'static str,
    },
    /// Input is not usable text (NUL byte: almost surely a binary file).
    Binary,
    /// Input is not valid UTF-8/ASCII text where text was required.
    BadEncoding,
}

/// A parse error with file and line context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    /// File being parsed (name or path supplied by the caller).
    pub file: String,
    /// 1-based line number, or 0 when no single line applies.
    pub line: usize,
    /// What went wrong.
    pub kind: ErrorKind,
}

impl Error {
    /// Build an error for one line of a file.
    #[must_use]
    pub fn new(file: &str, line: usize, kind: ErrorKind) -> Self {
        Self {
            file: file.to_string(),
            line,
            kind,
        }
    }

    /// Build an error not tied to one line.
    #[must_use]
    pub fn whole_file(file: &str, kind: ErrorKind) -> Self {
        Self::new(file, 0, kind)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.line == 0 {
            write!(f, "{}: {}", self.file, describe(&self.kind))
        } else {
            write!(f, "{}:{}: {}", self.file, self.line, describe(&self.kind))
        }
    }
}

impl std::error::Error for Error {}

fn describe(kind: &ErrorKind) -> String {
    match kind {
        ErrorKind::FieldCount { expected, found } => match expected {
            Some(e) => format!("want {e} fields, found {found}"),
            None => format!("bad field count ({found})"),
        },
        ErrorKind::BadScalar { field, want } => {
            format!("field {field} is not a {want}")
        }
        ErrorKind::Unterminated { section } => {
            format!("section `{section}` never closed with `end`")
        }
        ErrorKind::StrayEnd => "row or `end` outside any section".to_string(),
        ErrorKind::UnknownMarker { marker } => {
            format!("unknown marker `{marker}`")
        }
        ErrorKind::BadHeader { want } => format!("bad header (want {want})"),
        ErrorKind::Binary => "binary input (NUL byte), not a text table".to_string(),
        ErrorKind::BadEncoding => "input is not valid text".to_string(),
    }
}

/// Fallible result of every parser in this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Reject inputs that contain NUL bytes (binary files misrouted here).
pub(crate) fn check_text(file: &str, bytes: &[u8]) -> Result<()> {
    if bytes.contains(&0) {
        return Err(Error::whole_file(file, ErrorKind::Binary));
    }
    Ok(())
}

/// Decode bytes as text. The data files are ASCII with a few
/// Windows-1252 bytes (accented names in credits, glyph comments in fonts,
/// dashes in scrollbars), so bytes decode as Windows-1252: ASCII and
/// 0xA0-0xFF map to the same code points, 0x80-0x9F follow the standard
/// Windows table. NUL bytes still mean binary (see [`check_text`]).
pub(crate) fn decode(file: &str, bytes: &[u8]) -> Result<String> {
    check_text(file, bytes)?;
    Ok(decode_cp1252(bytes))
}

/// Decode Windows-1252 bytes to a string. Every byte maps; this cannot fail.
fn decode_cp1252(bytes: &[u8]) -> String {
    // 0x80-0x9F mappings; 0x81,0x8D,0x8F,0x90,0x9D are undefined in
    // Windows-1252 and become U+FFFD.
    const EXTRA: [u32; 32] = [
        0x20AC, 0xFFFD, 0x201A, 0x0192, 0x201E, 0x2026, 0x2020, 0x2021, 0x02C6, 0x2030, 0x0160,
        0x2039, 0x0152, 0xFFFD, 0x017D, 0xFFFD, 0xFFFD, 0x2018, 0x2019, 0x201C, 0x201D, 0x2022,
        0x2013, 0x2014, 0x02DC, 0x2122, 0x0161, 0x203A, 0x0153, 0xFFFD, 0x017E, 0x0178,
    ];
    let mut out = String::with_capacity(bytes.len());
    for &b in bytes {
        let c = match b {
            0x80..=0x9F => EXTRA[(b - 0x80) as usize],
            _ => u32::from(b),
        };
        out.push(char::from_u32(c).unwrap_or('\u{FFFD}'));
    }
    out
}
