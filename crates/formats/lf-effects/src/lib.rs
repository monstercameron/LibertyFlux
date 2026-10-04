//! Readers for the game's particle and visual effect definitions.
//!
//! This crate covers three shapes of effect data, all read from the owner's
//! own game files:
//!
//! * [`wpfl`]: the five binary effect packages (`*.wpfl`), which are RSC5
//!   resources holding banks of effect, emitter and particle rules keyed by
//!   name hash.
//! * [`fxdat`]: the ten text rule tables (`bloodFx.dat`, `entityFx.dat`,
//!   `explosionFx.dat`, `materialFx.dat`, `pedFx.dat`, `vehicleFx.dat`,
//!   `weaponFx.dat`, plus episode variants) that bind gameplay events and
//!   materials to named effects.
//! * [`emitter`]: the small XML emitter and renderer descriptions
//!   (`gtaRainEmitter.xml`, `gtaRainRender.xml` and the storm pair, each
//!   shipped twice).
//!
//! # Format description: `*.wpfl` packages
//!
//! A package is an RSC5 resource of type `0x24` (core and episode banks) or
//! `0x1B` (entity and script banks); see [`lf_resource`] for the container.
//! All integers below are little-endian. Offsets are byte offsets into the
//! decompressed system segment unless stated otherwise.
//!
//! ```text
//! root (system+0, 8 words):
//!   word   content
//!   0      bank pointer (system-segment tagged pointer)
//!   1      always the 0xCDCDCDCD debug fill (empty slot)
//!   2      bank pointer
//!   3      bank pointer
//!   4      always the 0xCDCDCDCD debug fill (empty slot)
//!   5      bank pointer
//!   6      bank pointer
//!   7      always null
//! ```
//!
//! Each bank is a hash-to-record dictionary with two parallel arrays:
//!
//! ```text
//! bank (8 words at the bank pointer):
//!   word   content
//!   0      vtable slot (raw executable address, differs per bank role)
//!   1-2    always zero
//!   3      always 1
//!   4      pointer to the hash array (u32 keys, count entries)
//!   5      count in both halves: low 16 bits = high 16 bits = entry count
//!   6      pointer to the record-pointer array (system pointers, count entries)
//!   7      same count word as word 5
//! ```
//!
//! Bank roles, from record shapes (slot 3 is verified, the rest inferred):
//!
//! ```text
//! slot   role (inferred)        record classes seen
//!   0    texture/sprite references  one class per package
//!   2    unknown (few entries)      one class per package
//!   3    emitter rules (verified)   two classes: sprite emitters, model emitters
//!   5    particle rules (inferred)  one class per package
//!   6    unknown                    one class per package
//! ```
//!
//! Slot 3 is verified as the emitter bank because its two record classes
//! split in exactly the count of the `ptxsprite` and `ptxmodel` type tags
//! stored in the same file, in all five packages. The `ptx` names live in
//! records of another bank, not inline in the slot-3 records.
//!
//! Effect names are plain ASCII strings stored NUL-terminated in the system
//! segment (for example the explosion names referenced by `explosionFx.dat`).
//! Bank keys are the Jenkins one-at-a-time hash of the entry name (see
//! [`hash`]); most keys resolve against the names found in the same file.
//!
//! # Format description: `*.dat` rule tables
//!
//! ```text
//! line 1      version string ("1.00", "2.00", "5.00", ...)
//! comments    lines starting with '#' (column headers live here)
//! tables      one or more NAME_START ... rows ... NAME_END blocks
//! rows        whitespace-separated fields; blank lines ignored
//! ```
//!
//! Every table in every shipped file has a fixed field count per table; the
//! parser preserves rows as strings and does not interpret fields.
//!
//! # Format description: emitter XML
//!
//! A tiny flat XML shape: an XML declaration, one root element naming the
//! engine type (`rage__ptxSimpleEmitter`, `rage__ptxgpuRenderSettings`), and
//! self-closing child elements whose attributes are the named properties.
//!
//! # What is still unknown
//!
//! * The exact roles of banks in slots 0, 2, 5 and 6.
//! * The record layouts behind each bank entry (emitter parameters,
//!   keyframed properties, texture/model references); records are exposed as
//!   raw bytes plus their vtable word for a future lane.
//! * Which bank holds the top-level named effects versus sub-rules.
//! * Some bank hashes do not resolve to any inline name; those names may be
//!   referenced only from gameplay data or scripts.
//!
//! # Sources
//!
//! No prose documentation of the binary layout was found: `RageLib` and
//! `SparkIV` name only the `0x1B`/`0x24` resource type ids and ship no
//! particle reader, the `GTAMods` wiki has no particle page, and the GTA
//! wiki's WPFL page could not be fetched. The layout above was established
//! by direct inspection of the five packages. The text tables and XML files
//! are self-describing. The Jenkins hash matches the one used for RPF3
//! entry names (see `lf-archive`).

pub mod emitter;
pub mod fxdat;
pub mod hash;
pub mod wpfl;

pub use emitter::{EmitterFile, Prop};
pub use fxdat::{FxFile, FxTable};
pub use hash::jenkins_oat;
pub use wpfl::{Bank, BankEntry, WpflFile};

/// Result type used throughout this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors returned when parsing effect data.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// The RSC5 container failed to parse.
    Resource(lf_resource::Error),
    /// The resource is not an effects package (expected type 0x1B or 0x24).
    NotEffects {
        /// The type id actually found.
        found: u32,
    },
    /// A root or bank word had an unexpected shape.
    BadLayout {
        /// What was wrong, for the error message.
        detail: String,
    },
    /// A pointer was null, mistagged or out of bounds.
    BadPointer {
        /// The raw pointer value.
        value: u32,
    },
    /// A declared count exceeds the crate's sanity cap.
    TooLarge {
        /// What was too large, for the error message.
        what: &'static str,
        /// The declared count.
        count: u64,
    },
    /// A text rule file or XML file was malformed.
    Malformed {
        /// One-based line number (0 when the file has no lines).
        line: usize,
        /// What was wrong, for the error message.
        detail: String,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Resource(e) => write!(f, "resource error: {e}"),
            Error::NotEffects { found } => {
                write!(f, "not an effects package: type {found:#x}")
            }
            Error::BadLayout { detail } => write!(f, "bad effects layout: {detail}"),
            Error::BadPointer { value } => {
                write!(f, "bad effects pointer: {value:#010x}")
            }
            Error::TooLarge { what, count } => {
                write!(f, "{what} too large: {count}")
            }
            Error::Malformed { line, detail } => {
                write!(f, "malformed effect text at line {line}: {detail}")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Resource(e) => Some(e),
            _ => None,
        }
    }
}

impl From<lf_resource::Error> for Error {
    fn from(e: lf_resource::Error) -> Self {
        Error::Resource(e)
    }
}
