//! Read-only parser for the two archive containers shipped with the game:
//! RPF (versions 2 and 3) and IMG (version 3).
//!
//! This crate was written from public format documentation and from the
//! author's own inspection of the game's files. It contains no game data.
//!
//! # Formats
//!
//! Both containers hold a table of contents followed by entry payloads.
//! All integers are little-endian. All parsing is done from byte slices with
//! explicit reads, so behaviour does not depend on pointer width or platform.
//!
//! ## RPF versions 2 and 3
//!
//! ```text
//! offset  size  field
//! 0x00    4     magic: "RPF2" or "RPF3"
//! 0x04    4     table-of-contents size in bytes
//! 0x08    4     number of records (files plus directories)
//! 0x0C    4     unknown, always zero in shipped files
//! 0x10    4     table encryption flag (nonzero means encrypted)
//! 0x14    4     content tag: nonzero only in the one archive whose file
//!               payloads are encrypted as well as the table
//! 0x18    0x7E8 reserved padding up to the table
//! 0x800   toc   table of contents (decrypted as one block when the flag
//!               at 0x10 is set; see [`crypto`])
//! ```
//!
//! The decrypted table holds `count` records of 16 bytes each, then a name
//! block (`toc_size - count * 16` bytes).
//!
//! A record is a directory when the signed 32-bit word at record offset 8 is
//! negative, otherwise a file:
//!
//! ```text
//! directory record:
//!   0x0  4  name reference (RPF2: byte offset into the name block;
//!                            RPF3: content hash, see below)
//!   0x4  4  unknown word
//!   0x8  4  index of the first child record (mask 0x7FFFFFFF)
//!   0xC  4  number of child records (mask 0x0FFFFFFF)
//!
//! file record:
//!   0x0  4  name reference (offset for RPF2, hash for RPF3)
//!   0x4  4  logical size in bytes (signed)
//!   0x8  4  payload offset, or packed resource offset (signed)
//!   0xC  4  control word
//! ```
//!
//! Control word decoding for file records:
//!
//! * If both top bits are set (`control & 0xC0000000 == 0xC0000000`) the
//!   entry is a resource: the real payload offset is `offset & 0x7FFFFF00`,
//!   the resource type id is the low byte of the offset, and the on-disk
//!   size equals the logical size.
//! * Otherwise the on-disk size is `control & 0xBFFFFFFF` and bit
//!   `0x40000000` means the payload is raw-deflate compressed.
//!
//! RPF2 name references are byte offsets into the name block, which holds
//! real null-terminated names. RPF3 name references are 32-bit Jenkins
//! one-at-a-time hashes of the entry name (see [`hash`]); the name block
//! holds only the root "/" plus padding, so names must be resolved through
//! an external hash-to-name table (see
//! [`RpfArchive::resolve_names`](rpf::RpfArchive::resolve_names)).
//!
//! ## IMG version 3
//!
//! ```text
//! offset  size  field
//! 0x00    4     magic 0xA94E2A52 (encrypted on disk when the table is)
//! 0x04    4     version, always 3
//! 0x08    4     number of entries
//! 0x0C    4     table size in bytes
//! 0x10    2     record size, always 16
//! 0x12    2     unknown (253 of 254 shipped files hold 233)
//! 0x14    toc   table: `count` records of 16 bytes, then names
//! ```
//!
//! In encrypted files the first 16 header bytes and the whole table region
//! are encrypted with the same cipher as RPF tables; bytes 16-19 of the
//! header stay plaintext. A file is detected as encrypted when the first
//! word differs from the magic.
//!
//! ```text
//! entry record:
//!   0x0  4  size, or resource flags when either top bit is set
//!   0x4  4  resource type id (signed)
//!   0x8  4  payload offset in 0x800-byte blocks (signed)
//!   0xC  2  payload length in 0x800-byte blocks
//!   0xE  2  flags; the low 11 bits are the padding count
//! ```
//!
//! Resource payloads are `blocks * 0x800 - padding` bytes long and start
//! with the RSC container (magic "RSC", version 5); plain entries are `w0`
//! bytes long. IMG has no directories: names are sequential
//! null-terminated strings after the records, one per entry in order.
//!
//! ## Encryption
//!
//! The table cipher is AES-256 in ECB mode applied 16 times in succession
//! over the 16-byte-aligned prefix; trailing bytes under 16 pass through
//! unchanged. The 32-byte key lives in the owner's game executable and is
//! located at run time by the published method: the 32-byte window whose
//! SHA-1 matches the published key hash (see [`crypto::find_key`]). The key
//! is kept in memory only and is never printed, logged or stored by this
//! crate.
//!
//! ## What is still unknown
//!
//! * The RPF header word at 0x0C (always zero) and the directory-record word
//!   at offset 4 have unknown meaning.
//! * The IMG header word at 0x12 has unknown meaning (one shipped file
//!   differs from the rest).
//! * RPF3 directory name hashes do not resolve through public name tables;
//!   the root often carries a filler value instead of a hash.
//! * No shipped RPF entry uses the deflate flag, so compressed payloads are
//!   exercised by unit tests only.
//!
//! # Example
//!
//! ```no_run
//! use lf_archive::{Archive, crypto, rpf::RpfArchive};
//! use std::fs::File;
//! use std::io::BufReader;
//!
//! let key = crypto::load_key_from_exe("GTAIV.exe")?;
//! let mut reader = BufReader::new(File::open("audio.rpf")?);
//! let archive = RpfArchive::open(&mut reader, Some(&key))?;
//! for entry in archive.entries() {
//!     println!("{} {}", entry.size, entry.path);
//! }
//! # Ok::<(), lf_archive::Error>(())
//! ```

use std::io::{Read, Seek};

pub mod crypto;
pub mod hash;
pub mod img;
pub mod rpf;

pub use crypto::Key;

/// Errors returned when parsing or reading archives.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// An underlying I/O operation failed.
    Io(std::io::Error),
    /// The magic bytes did not match any known container.
    BadMagic {
        /// What was expected, for the error message.
        expected: &'static str,
        /// The bytes actually found (up to 4).
        found: Vec<u8>,
    },
    /// A version number this crate does not support.
    UnsupportedVersion {
        /// Container name, for the error message.
        format: &'static str,
        /// The version number found.
        version: u32,
    },
    /// The table of contents is encrypted but no key was supplied.
    EncryptedTable,
    /// A file payload is encrypted but no key was supplied.
    EncryptedContent,
    /// The file ended before the structure was complete.
    Truncated(&'static str),
    /// A table record was internally inconsistent.
    BadEntry(String),
    /// An entry's payload range lies outside the file.
    EntryOutOfRange {
        /// Entry path, for the error message.
        path: String,
    },
    /// A declared size exceeds the crate's sanity cap.
    TooLarge {
        /// What was too large, for the error message.
        what: &'static str,
        /// The declared size.
        size: u64,
    },
    /// Decompression of a payload failed.
    Decompress(String),
    /// No key with the published hash was found in the executable.
    KeyNotFound,
    /// A directory tree did not cover the entry list as expected.
    BadTree(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "i/o error: {e}"),
            Error::BadMagic { expected, found } => {
                write!(f, "bad magic: expected {expected}, found {found:02X?}")
            }
            Error::UnsupportedVersion { format, version } => {
                write!(f, "unsupported {format} version {version}")
            }
            Error::EncryptedTable => {
                write!(f, "table of contents is encrypted but no key was given")
            }
            Error::EncryptedContent => write!(f, "entry payload is encrypted but no key was given"),
            Error::Truncated(what) => write!(f, "truncated file while reading {what}"),
            Error::BadEntry(msg) => write!(f, "bad table entry: {msg}"),
            Error::EntryOutOfRange { path } => write!(f, "entry payload outside file: {path}"),
            Error::TooLarge { what, size } => write!(f, "{what} too large: {size}"),
            Error::Decompress(msg) => write!(f, "decompression failed: {msg}"),
            Error::KeyNotFound => write!(f, "no key with the published hash found"),
            Error::BadTree(msg) => write!(f, "bad directory tree: {msg}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

/// Shorthand for `Result<T, lf_archive::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// Whether an [`Entry`] is a file or a directory.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryKind {
    /// A file with a payload that can be read.
    File,
    /// A directory (RPF only; IMG archives are flat).
    Directory,
}

/// Resource metadata for entries flagged as engine resources.
///
/// Resource payloads use the RSC container; this struct records only what
/// the archive table says about the entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceInfo {
    /// Engine resource type id.
    pub type_id: i32,
    /// Raw flags word from the table record.
    pub flags: u32,
}

/// One entry in an archive, in container-independent form.
///
/// Paths use `/` separators and start at the archive root (for example
/// `/data/taskparams.txt`). Entries whose names are unknown (unresolved RPF3
/// hashes) carry a placeholder path of the form `/<hash:061B65DA>` and
/// [`Entry::name`] is `None`.
#[derive(Clone, Debug)]
pub struct Entry {
    /// Full path within the archive.
    pub path: String,
    /// File or directory name when known.
    pub name: Option<String>,
    /// RPF3 content hash, when the container stores hashes instead of names.
    pub hash: Option<u32>,
    /// File or directory.
    pub kind: EntryKind,
    /// Logical (uncompressed) payload size in bytes. Zero for directories.
    pub size: u64,
    /// Payload bytes stored in the archive. Zero for directories.
    pub stored_size: u64,
    /// File offset of the payload. Zero for directories.
    pub offset: u64,
    /// Whether the stored payload is raw-deflate compressed.
    pub compressed: bool,
    /// Resource metadata when the table flags the entry as a resource.
    pub resource: Option<ResourceInfo>,
}

impl Entry {
    /// True for files, false for directories.
    #[must_use]
    pub fn is_file(&self) -> bool {
        self.kind == EntryKind::File
    }

    /// True for directories, false for files.
    #[must_use]
    pub fn is_dir(&self) -> bool {
        self.kind == EntryKind::Directory
    }
}

/// A parsed archive table of contents with on-demand payload reads.
///
/// Implementations hold the parsed table only; payload bytes are read from
/// the caller-supplied reader so multi-gigabyte archives never need to be
/// loaded fully into memory.
pub trait Archive {
    /// All entries: files and, for RPF, directories.
    fn entries(&self) -> &[Entry];

    /// Number of entries.
    fn len(&self) -> usize {
        self.entries().len()
    }

    /// True when the archive holds no entries.
    fn is_empty(&self) -> bool {
        self.entries().is_empty()
    }

    /// Index of the entry with the given path, if any.
    fn find(&self, path: &str) -> Option<usize> {
        self.entries().iter().position(|e| e.path == path)
    }

    /// Read an entry's stored bytes.
    ///
    /// `key` is required only when the archive encrypts file payloads (the
    /// single RPF with the content tag set); table decryption already
    /// happened at open time. Compressed payloads are returned still
    /// compressed; use [`Archive::read_file`] for the plain bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    fn read_raw<R: Read + Seek>(
        &self,
        reader: &mut R,
        index: usize,
        key: Option<&Key>,
    ) -> Result<Vec<u8>>;

    /// Read an entry's logical bytes, decompressing when flagged.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    fn read_file<R: Read + Seek>(
        &self,
        reader: &mut R,
        index: usize,
        key: Option<&Key>,
    ) -> Result<Vec<u8>>;
}

/// Which container a file holds, decided from its first bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// RPF versions 2 or 3 (plaintext magic).
    Rpf,
    /// IMG version 3 with a plaintext header (unencrypted).
    Img,
    /// IMG version 3 with an encrypted header (magic unreadable).
    EncryptedImg,
}

/// Classify a file from its first 20 bytes.
///
/// `header` must hold at least 4 bytes; 20 bytes allow the IMG plaintext
/// check. Encrypted IMG files are reported as [`Kind::EncryptedImg`] only
/// when a `key` is supplied for the trial decryption, otherwise `None`.
#[must_use]
pub fn detect(header: &[u8], key: Option<&Key>) -> Option<Kind> {
    if header.len() >= 4 && (header[0..4] == *b"RPF2" || header[0..4] == *b"RPF3") {
        return Some(Kind::Rpf);
    }
    if header.len() >= 20 {
        let magic = u32::from_le_bytes([header[0], header[1], header[2], header[3]]);
        if magic == img::IMG_MAGIC {
            return Some(Kind::Img);
        }
        if let Some(k) = key {
            let mut trial = [0u8; 16];
            trial.copy_from_slice(&header[0..16]);
            crypto::decrypt_tables(&mut trial, k);
            let magic = u32::from_le_bytes([trial[0], trial[1], trial[2], trial[3]]);
            let version = u32::from_le_bytes([trial[4], trial[5], trial[6], trial[7]]);
            if magic == img::IMG_MAGIC && version == img::IMG_VERSION {
                return Some(Kind::EncryptedImg);
            }
        }
    }
    None
}

/// An opened archive of either container kind.
#[derive(Debug)]
pub enum AnyArchive {
    /// An RPF version 2 or 3 archive.
    Rpf(rpf::RpfArchive),
    /// An IMG version 3 archive.
    Img(img::ImgArchive),
}

impl Archive for AnyArchive {
    fn entries(&self) -> &[Entry] {
        match self {
            AnyArchive::Rpf(a) => a.entries(),
            AnyArchive::Img(a) => a.entries(),
        }
    }

    fn read_raw<R: Read + Seek>(
        &self,
        reader: &mut R,
        index: usize,
        key: Option<&Key>,
    ) -> Result<Vec<u8>> {
        match self {
            AnyArchive::Rpf(a) => a.read_raw(reader, index, key),
            AnyArchive::Img(a) => a.read_raw(reader, index, key),
        }
    }

    fn read_file<R: Read + Seek>(
        &self,
        reader: &mut R,
        index: usize,
        key: Option<&Key>,
    ) -> Result<Vec<u8>> {
        match self {
            AnyArchive::Rpf(a) => a.read_file(reader, index, key),
            AnyArchive::Img(a) => a.read_file(reader, index, key),
        }
    }
}

/// Open either container kind, detecting from the file header.
///
/// The reader is rewound before reading. `key` is required for encrypted
/// tables (every shipped RPF and all but four shipped IMGs).
///
/// # Errors
///
/// Returns an error if the input is truncated or malformed.
pub fn open<R: Read + Seek>(reader: &mut R, key: Option<&Key>) -> Result<AnyArchive> {
    use std::io::SeekFrom;
    reader.seek(SeekFrom::Start(0))?;
    let mut header = [0u8; 20];
    read_exact_or(reader, &mut header, "header")?;
    match detect(&header, key) {
        Some(Kind::Rpf) => Ok(AnyArchive::Rpf(rpf::RpfArchive::open(reader, key)?)),
        Some(Kind::Img | Kind::EncryptedImg) => {
            Ok(AnyArchive::Img(img::ImgArchive::open(reader, key)?))
        }
        None => Err(Error::BadMagic {
            expected: "RPF2, RPF3 or IMG v3",
            found: header[0..4].to_vec(),
        }),
    }
}

/// Read exactly `buf.len()` bytes, mapping short reads to [`Error::Truncated`].
pub(crate) fn read_exact_or<R: Read>(
    reader: &mut R,
    buf: &mut [u8],
    what: &'static str,
) -> Result<()> {
    reader.read_exact(buf).map_err(|e| {
        if e.kind() == std::io::ErrorKind::UnexpectedEof {
            Error::Truncated(what)
        } else {
            Error::Io(e)
        }
    })
}

/// Length of the stream, or `None` when the stream cannot report it.
pub(crate) fn stream_len<R: Seek>(reader: &mut R) -> Option<u64> {
    let pos = reader.stream_position().ok()?;
    let end = reader.seek(std::io::SeekFrom::End(0)).ok()?;
    reader.seek(std::io::SeekFrom::Start(pos)).ok()?;
    Some(end)
}

/// Inflate a raw-deflate payload, bounded by the table's inflated size.
pub(crate) fn inflate_raw(data: &[u8], size: u64) -> Result<Vec<u8>> {
    use flate2::read::DeflateDecoder;
    let mut out = Vec::new();
    DeflateDecoder::new(data)
        .take(size.saturating_add(1))
        .read_to_end(&mut out)
        .map_err(|e| Error::Decompress(e.to_string()))?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_rpf() {
        assert_eq!(detect(b"RPF2xxxxxxxxxxxxxxxx", None), Some(Kind::Rpf));
        assert_eq!(detect(b"RPF3xxxxxxxxxxxxxxxx", None), Some(Kind::Rpf));
    }

    #[test]
    fn detect_open_img() {
        let mut h = [0u8; 20];
        h[0..4].copy_from_slice(&img::IMG_MAGIC.to_le_bytes());
        assert_eq!(detect(&h, None), Some(Kind::Img));
    }

    #[test]
    fn detect_unknown() {
        assert_eq!(detect(b"ZZZZxxxxxxxxxxxxxxxx", None), None);
        assert_eq!(detect(b"RP", None), None);
    }
}
