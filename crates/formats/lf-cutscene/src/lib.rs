//! Read-only parser for cutscene description (`.cut`) files.
//!
//! Cutscenes are stored as plain-text tag files, one per cutscene, inside the
//! `cuts.img` animation archives (one archive per episode). Each file holds one
//! or more cutscenes; each cutscene has file-level tags (subtitles, flags,
//! draw distance, blocking bounds and others) followed by one or more
//! animation sections delimited by `[SECTION_START]` / `[SECTION_END]` markers.
//! Sections name the `.wad` animation file that plays, the models and their
//! animations, the camera, the audio bank, the timing and the placement.
//!
//! This crate was written from the public `GTAMods` wiki Cutscene page and from
//! the author's own inspection of the game's files. It contains no game data.
//!
//! # Format description
//!
//! All integers and floats are ASCII text. All multi-byte framing (the archive
//! layer) is little-endian; see [`lf_archive`]. The `.cut` payload itself is
//! text, so this crate is pointer-width and platform independent.
//!
//! ## File layout
//!
//! ```text
//! [CUTSCENE_HEADER] ... [/CUTSCENE_HEADER]   frame boundaries, one line
//! [FLAGS] ... [/FLAGS]                       flag names, one per line
//! [BLOCKING_BOUNDS] ... [/BLOCKING_BOUNDS]   13 floats per line
//! [CAMCORDER] ... [/CAMCORDER]                start/end ms pairs
//! [DRAW_DISTANCE] ... [/DRAW_DISTANCE]        time, second, distance[, unknown]
//! [MISSION_TEXT_NAME] ... [/MISSION_TEXT_NAME] audio label, one line
//! [TEXT] ... [/TEXT]                         start_ms, len_ms, gxt_key
//! [PLAYER_START] ... [/PLAYER_START]          x y z world position
//! [VARIATION] ... [/VARIATION]                4 or 5 integers per line
//! [PROPS] ... [/PROPS]                       model_id index prop
//! [MAX_PEDS] / [MAX_CARS]                    one integer
//! [TIMECYCLE_MODIFIER_NAME]                  one name
//! [SECTION_START]                            marker, no close tag
//!   [MODELS] ... [/MODELS]                   id model anim [head_anim] [unk]
//!   [COMPRESSION] ... [/COMPRESSION]          model_id mode_letter
//!   [LIGHTS] ... [/LIGHTS]                   animation names from the .wad
//!   [EFFECTS] ... [/EFFECTS]                 12 fields plus optional extras
//!   [VEHICLE_DETAILS] ... [/VEHICLE_DETAILS] id + 4 colours + one value
//!   [VEHICLE_REMOVAL] ... [/VEHICLE_REMOVAL]  model_index bone_id
//!   [ORIENT] ... [/ORIENT]                   heading in degrees
//!   [REMOVE] ... [/REMOVE]                   x y z model [unknown]
//!   [TIME] ... [/TIME]                       always empty in shipped files
//!   [OFFSET] ... [/OFFSET]                   x y z cutscene origin
//!   [DURATION] ... [/DURATION]                section length in milliseconds
//!   [AUDIO] ... [/AUDIO]                     audio bank name in cutscenes.rpf
//!   [ANIM] ... [/ANIM]                       .wad file stem in cuts.img
//!   [ANIMRANGE] ... [/ANIMRANGE]              "range on" + first/last frame
//!   [CAMERA] ... [/CAMERA]                   camera animation name
//! [SECTION_END]                              marker, no close tag
//! ... repeated groups and sections ...
//! <binary slack: zero padding and/or uninitialised-memory fill to EOF>
//! ```
//!
//! ## Parsing rules
//!
//! * Lines end with LF or CRLF. Blank lines are ignored everywhere.
//! * An open tag implicitly closes any open tag (shipped files rely on this).
//! * `[SECTION_START]` and `[SECTION_END]` are bare markers: they never have
//!   close tags.
//! * `CUTSCENE_HEADER` values are ascending frame numbers; shipped files carry
//!   exactly one more value than the cutscene has sections, except for one
//!   subtitles-only cutscene that has none.
//! * `TIME` sections are always empty in shipped files; the parser keeps any
//!   lines it finds there verbatim.
//! * Unknown tags are preserved verbatim (see [`cut::UnknownSection`]); rows
//!   that fail typed parsing are preserved in `bad_rows` with a reason.
//! * Anything that looks wrong but recoverable is reported in
//!   [`cut::CutsceneFile::warnings`], never silently dropped and never fatal.
//! * Parsing stops at the first line containing interior NUL bytes or invalid
//!   UTF-8: shipped payloads are padded to a 2048-byte boundary with zero
//!   bytes and uninitialised-memory fill after the last text line. Trailing
//!   NUL bytes on an otherwise text line are trimmed (one shipped file ends
//!   its final marker that way), and stray non-tag lines after the final tag
//!   are treated as slack. The skipped tail is counted in
//!   [`cut::CutsceneFile::trailing_slack_len`].
//!
//! ## Units
//!
//! Times and durations are milliseconds. `DURATION` values are fractional
//! (`45466.667969`). `CUTSCENE_HEADER` and `ANIMRANGE` values are animation
//! frame numbers. `DRAW_DISTANCE` rows carry a start time, a second value
//! (ignored by the game per the wiki; shipped files use either a running
//! index or an end time), a draw distance and an optional fourth value that
//! defaults to `0.05`.
//!
//! ## Cross-references (see [`catalog`])
//!
//! * `ANIM` names a `.wad` entry in the same `cuts.img` (case-insensitive).
//! * `MODELS` names resolve against `cutsprops.img` of the same episode or the
//!   base game; `player` lives in `playerped.rpf` instead and ambient ped and
//!   vehicle names live in their own archives.
//! * `AUDIO` names an entry in `cutscenes.rpf` (hash-named; see
//!   [`catalog::audio_hash_candidates`]).
//! * `TEXT` keys name strings in the `.gxt` text databases.
//!
//! ## What is still unknown
//!
//! * The `FIXUP` and `ATTACHMENT` tags from the wiki never occur in shipped
//!   files, so their row shapes are unconfirmed.
//! * The meaning of `COMPRESSION` mode letters beyond `A`, the fifth
//!   `VARIATION` value, the `TIME` tag, and the trailing values of `EFFECTS`,
//!   `REMOVE`, `MODELS` and `VEHICLE_DETAILS` rows.
//! * Whether the `DRAW_DISTANCE` second value is an end time or an index (both
//!   conventions ship; the game ignores it either way).
//! * The `.wad` animation container internals (animation lane's area; this
//!   crate only checks the `RSC5` resource header via [`lf_resource`]).
//!
//! ## Sources
//!
//! Layout learned from the `GTAMods` wiki Cutscene page (GTA IV tags section)
//! plus direct measurement of all 344 shipped files. No open-source `.cut`
//! parser was found in the RageLib/SparkIV, `GTA4Unity` or gta4-webmap sources
//! searched, so the row shapes for the tags the wiki leaves blank
//! (`EFFECTS`, `LIGHTS`, `ORIENT`, `REMOVE`, `VARIATION`, `COMPRESSION`,
//! `TIME`, `ANIMRANGE`, `PLAYER_START`, `CUTSCENE_HEADER`) were derived from
//! the files themselves. No GPL code was copied.
//!
//! # Example
//!
//! ```no_run
//! use lf_cutscene::cut::CutsceneFile;
//!
//! let bytes = std::fs::read("intro.cut")?;
//! let file = CutsceneFile::parse(&bytes)?;
//! for group in &file.groups {
//!     let ms: f32 = group.sections.iter().flat_map(|s| &s.durations_ms).sum();
//!     println!("{} sections, {:.0} ms", group.sections.len(), ms);
//! }
//! for warning in &file.warnings {
//!     eprintln!("line {}: {}", warning.line, warning.message);
//! }
//! # Ok::<(), lf_cutscene::Error>(())
//! ```

pub mod catalog;
pub mod cut;

pub use cut::CutsceneFile;

/// Errors returned when parsing or reading cutscene files.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// An underlying I/O operation failed.
    Io(std::io::Error),
    /// An archive operation failed (see [`lf_archive::Error`]).
    Archive(lf_archive::Error),
    /// A resource header failed to parse (see [`lf_resource::Error`]).
    Resource(lf_resource::Error),
    /// The input exceeds the crate's sanity cap (64 MiB).
    TooLarge {
        /// The declared size in bytes.
        size: u64,
    },
    /// A required file or archive entry was not found.
    NotFound(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "i/o error: {e}"),
            Error::Archive(e) => write!(f, "archive error: {e}"),
            Error::Resource(e) => write!(f, "resource error: {e}"),
            Error::TooLarge { size } => write!(f, "input too large: {size} bytes"),
            Error::NotFound(what) => write!(f, "not found: {what}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Archive(e) => Some(e),
            Error::Resource(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<lf_archive::Error> for Error {
    fn from(e: lf_archive::Error) -> Self {
        Error::Archive(e)
    }
}

impl From<lf_resource::Error> for Error {
    fn from(e: lf_resource::Error) -> Self {
        Error::Resource(e)
    }
}

/// Shorthand for `Result<T, lf_cutscene::Error>`.
pub type Result<T> = std::result::Result<T, Error>;
