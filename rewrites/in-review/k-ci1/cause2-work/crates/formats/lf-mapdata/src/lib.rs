//! Readers for map definition and placement files: `.ide`, `.ipl`, `.wpl`,
//! and `gta.dat`-style load lists.
//!
//! # Formats
//!
//! All three map formats are little-endian and plain on disk: no compression,
//! no encryption, no archive key needed. The text formats (`.ide`, `.ipl`)
//! share one shape: a sequence of named sections, each a list of
//! comma-separated records terminated by a line reading `end`:
//!
//! ```text
//! objs
//! myprop, mytxd, 100, 0, 0, -1, -1, -1, 1, 1, 1, 0, 0, 0, 2, null
//! end
//! ```
//!
//! A `#` starts a comment that runs to the end of the line, both on its own
//! line and after data. Blank lines are ignored. Section names are matched
//! case-insensitively by this crate (the game is case-sensitive; shipped files
//! use lower case throughout).
//!
//! ## Field splitting
//!
//! The game is documented to replace commas with spaces and scan each line
//! with `sscanf`-style token parsing, which means a missing comma between two
//! values still parses (whitespace separates them) and a doubled comma is
//! harmless. Several shipped rows rely on this: they are missing a comma or
//! carry a stray empty field. This crate therefore splits each record line on
//! commas into fields, and the typed parsers for the affected sections
//! ([`ide::Cars`], [`ide::Peds`], [`ide::Weap`]) additionally split fields on
//! whitespace, so both spellings are accepted. Double-quoted fields keep any
//! commas inside them; the surrounding quotes are removed.
//!
//! The game reads at most 256 characters per line. A few shipped rows are
//! longer than that and are truncated mid-field; affected trailing fields are
//! therefore optional in the typed records.
//!
//! ## IDE (item definitions, `.ide`)
//!
//! One row per object the map system can place. Sections observed in shipped
//! files, with record shapes (counts are per record; `str` = text,
//! `f` = float, `i` = integer):
//!
//! | section | records | shape |
//! |---|---|---|
//! | `objs` | static objects | str, str, f, i, i, 6f box, 4f sphere, str (16 fields; 3 legacy rows carry only 4) |
//! | `tobj` | timed objects | `objs` shape plus one integer time mask (17 fields) |
//! | `anim` | animated objects | str, str, str, f, i, i, 6f box, 4f sphere, str (17 fields) |
//! | `tanm` | timed animated objects | `anim` shape plus one integer time mask (18 fields) |
//! | `cars` | vehicles | 7 str, 2 i, 2 f, 1 f, 1 i, 1 f, 1 i, optional flags word (15-16 tokens) |
//! | `peds` | pedestrians | 8 str, 1 i, 2 str, 2 i, 3 str (15-16 tokens; voice may be truncated away) |
//! | `weap` | weapons | 3 str, 3 i (6 tokens) |
//! | `txdp` | texture-dictionary parents | 2 str |
//! | `amat` | asset materials | str, i, str |
//! | `agrps` | asset groups | str, str, i |
//! | `hier` | hierarchy entries | 3-4 str plus one float radius (4-5 tokens) |
//! | `mlo` | interior room data | mixed: 8-field headers, 11-field placements, marker words, numeric lists |
//! | `2dfx` | attached effects | str, 3f position, integer kind, kind-dependent payload |
//! | `tree`, `path` | present but always empty in shipped files | - |
//!
//! The `mlo` numeric lists and several `hier`/`amat`/`weap` fields have no
//! public documentation; they are exposed as typed values with their meaning
//! marked unknown. See [`ide`] for the record types.
//!
//! ## IPL (text placement, `.ipl`)
//!
//! Human-readable placement. Every shipped file declares the same long list of
//! sections (`inst`, `cull`, `path`, `grge`, `enex`, `pick`, `cars`, `jump`,
//! `tcyc`, `auzo`, `blok`, `mult`, `vnod`, `link`, `pnod`, `plnk`, `mlo+`,
//! `2dfx`, `lodm`, `slow`, `occl`, `rtfx`) but only six ever carry records:
//!
//! | section | records | shape |
//! |---|---|---|
//! | `blok` | source-file stamp | 3 str, 2 i, 1 str (6 fields) |
//! | `cull` | cull zone (1 record shipped) | 11 numeric fields, meanings uncertain |
//! | `occl` | occlusion volumes | 9 f plus 1 i |
//! | `vnod` | path nodes | 3 f plus 10-11 int/uint/float values |
//! | `link` | path links | 6 integers |
//! | `2dfx` | placed effects | str, 3f, integer kind, kind-dependent payload |
//!
//! Instance placement moved to the binary `.wpl` files; the text `inst`
//! sections are always empty. See [`ipl`].
//!
//! ## WPL (binary placement, `.wpl`)
//!
//! Flat little-endian binary, not an RSC5 resource. A 68-byte header (a
//! version word, always 3, plus sixteen per-section record counts) is followed
//! by the record arrays in a fixed order that is *not* numeric section order:
//! `inst` (0), `grge` (2), `cars` (3), `tcyc` (4), `mlop` (8), `blok` (15),
//! `lodm` (9), `slow` (10). Sections 1, 5, 6, 7 and 11-14 are always zero.
//! Sections `cars` and `mlop` are always empty in shipped files, so their
//! documented layouts are implemented but unverified.
//!
//! | section | record size | contents |
//! |---|---|---|
//! | `inst` | 48 | position, quaternion rotation, model hash, flags, LOD index, two unknown fields |
//! | `grge` | 48 | two corners plus front vector, door/garage types, 8-byte name |
//! | `cars` | 56 | documented layout only, never observed |
//! | `tcyc` | 44 | two corners, four unknown words, box hash |
//! | `mlop` | 64 | documented layout only, never observed |
//! | `blok` | 132 | unknown word, 92-byte source-line string, unknown word, 8 unknown floats |
//! | `lodm` | 388 | bounding box, entry count, 10 hashes, 10 model names |
//! | `slow` | 24 | bounding box (two corners) |
//!
//! One shipped file carries trailing zero padding past the last record; the
//! parser keeps trailing bytes in [`wpl::WplFile::trailing_bytes`] instead of
//! failing. See [`wpl`].
//!
//! ## Load lists (`gta.dat`, `default.dat`, `images.txt`)
//!
//! Whitespace-separated `KEYWORD args...` lines with `#` comments. `gta.dat`
//! lists `IDE` and `IPL` files in load order plus `IMGLIST` and `WATER` lines;
//! `default.dat` binds tuning files. See [`loadlist`].
//!
//! # Sources
//!
//! Layouts were checked against the shipped files themselves and against the
//! public `GTAMods` wiki pages for Item Definition, Item Placement, WPL, and the
//! IDE section pages, read October 2026. No open-source parser code was copied;
//! RageLib/SparkIV sources were not consulted. Where the wiki and the files
//! disagree (the `tcyc` record size, the on-disk section order), the files win
//! and the difference is noted in the item documentation.
//!
//! # Errors
//!
//! Every parse function returns [`Error`] rather than panicking. Text parsing
//! additionally collects per-record failures: [`ide::IdeFile::errors`] and
//! [`ipl::IplFile::errors`] hold the records that did not match their typed
//! shape, so one bad row never hides the rest of the file.

pub mod ide;
pub mod ipl;
pub mod loadlist;
pub mod text;
pub mod wpl;

pub use text::{TextFile, TextRecord, TextSection};

use std::fmt;

/// Error type for all parsing in this crate.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// Input is not valid UTF-8 (text formats only).
    Utf8 {
        /// Byte offset where decoding failed, if known.
        offset: Option<usize>,
    },
    /// A text record has the wrong number of fields/tokens.
    FieldCount {
        /// 1-based line number.
        line: usize,
        /// Section name.
        section: String,
        /// How many fields or tokens were found.
        found: usize,
        /// What was expected, as human text.
        expected: String,
    },
    /// A text field did not parse as the expected number.
    BadNumber {
        /// 1-based line number.
        line: usize,
        /// Section name.
        section: String,
        /// Field index within the record.
        field: usize,
        /// The offending text.
        text: String,
    },
    /// Binary input ended before a complete header or record.
    Truncated {
        /// Byte offset where input ran out.
        offset: usize,
        /// What was being read.
        what: String,
    },
    /// Unsupported binary version word.
    BadVersion {
        /// The version word found.
        found: u32,
    },
    /// A load-list line has no keyword.
    EmptyDirective {
        /// 1-based line number.
        line: usize,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Utf8 { offset } => match offset {
                Some(o) => write!(f, "invalid UTF-8 at byte {o}"),
                None => write!(f, "invalid UTF-8"),
            },
            Error::FieldCount {
                line,
                section,
                found,
                expected,
            } => write!(
                f,
                "line {line} section '{section}': {found} fields, expected {expected}"
            ),
            Error::BadNumber {
                line,
                section,
                field,
                text,
            } => write!(
                f,
                "line {line} section '{section}' field {field}: '{text}' is not a number"
            ),
            Error::Truncated { offset, what } => {
                write!(f, "truncated at byte {offset} while reading {what}")
            }
            Error::BadVersion { found } => write!(f, "unsupported version {found}"),
            Error::EmptyDirective { line } => write!(f, "line {line}: empty directive"),
        }
    }
}

impl std::error::Error for Error {}
