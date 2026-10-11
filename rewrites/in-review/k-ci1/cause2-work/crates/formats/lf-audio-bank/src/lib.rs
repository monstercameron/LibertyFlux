//! Read-only parser for the PC audio of GTA IV (verified against 1.2.0.59).
//!
//! The game keeps audio in three layers:
//!
//! 1. **RPF3 archives** (`.rpf` under `pc/audio/sfx`): one table of contents
//!    per archive, AES-encrypted, listing nameless entries by 32-bit hash.
//!    Archives are opened with the `lf-archive` crate; this crate parses the
//!    bank and streamed payloads found inside them.
//! 2. **Bank** or **streamed** containers inside the archives: a bank holds
//!    many short sounds (speech lines, effects); a streamed file holds one
//!    long sound (a radio track, weather loop) split into seekable blocks.
//!    Both describe each sound with the same wave header. See [`bank`] and
//!    [`streamed`].
//! 3. **IMA 4-bit ADPCM** payload bytes, two samples per byte, decoded in
//!    software. See [`adpcm`].
//!
//! A fourth piece, the **versioned audio configuration** files
//! (`categories.dat15`, `curves.dat12`, `effects.dat11`, `game.dat16`,
//! `sounds.dat15`), holds the mixer/object database this crate parses
//! structurally (header, object region, name region). See [`dat`].
//!
//! # Format notes
//!
//! All integers are little-endian. All parsers borrow their input and never
//! copy; offsets are validated before every read and every failure is a
//! returned [`Error`], never a panic.
//!
//! ## Bank container
//!
//! ```text
//! off  size  field
//! 0x00 8     table: byte offset of the stream table (28 in every file seen)
//! 0x08 4     records_end: byte offset where the packed info records end
//! 0x0C 4     reserved (0 in every file seen)
//! 0x10 4     count: number of streams
//! 0x14 4     unknown (0 in most files; equals count in one speech bank seen)
//! 0x18 4     base: byte offset where ADPCM data starts
//! ```
//!
//! The stream table at `table` holds `count` entries of 16 bytes:
//! `info_off` (u64, byte offset of this stream's info record relative to the
//! end of the table), `name_hash` (u32), `record_len` (u32, length in bytes of
//! the info record including its trailer). Records are packed back to back in
//! file order, which usually differs from table order.
//!
//! Each info record starts with a 32-byte wave header:
//!
//! ```text
//! off  size  field
//! +0x00 8    data_off: byte offset of the sound's ADPCM bytes, relative to base
//! +0x08 4    name_hash (repeats the table entry)
//! +0x0C 4    size: ADPCM bytes, equal to sample_count / 2 rounded down
//! +0x10 4    sample_count
//! +0x14 4    flags (0 or all-ones in files seen; meaning unknown)
//! +0x18 2    sample_rate in Hz
//! +0x1A 2    unknown
//! +0x1C 4    codec: 0x400 IMA ADPCM, 0x1 16-bit PCM (`size` is then
//!            `2 * sample_count`), 0x200 Ogg Vorbis (one stream in the game)
//! ```
//!
//! The remaining `record_len - 32` bytes (trailer) are a 21- or 24-byte
//! header plus 3 bytes per 2048-byte data sector (`sectors` is the sound's
//! data rounded up; PCM sounds have no trailer). The trailer opens with the
//! words 56, 0 and the sample count; what selects the 24-byte header and
//! the remaining contents are not understood yet. Bytes between
//! `records_end` and `base` are zero padding. Sound data starts are multiples
//! of 2048.
//!
//! ## Streamed container
//!
//! ```text
//! off  size  field
//! 0x00 8     table_off: byte offset of the per-block file table
//! 0x08 4     blocks: number of data blocks
//! 0x0C 4     block_chunk: bytes per block on disk
//! 0x10 4     stream_count: always 0 (this word tells banks from streams)
//! 0x14 8     ch_table_off: byte offset of the channel region (48 seen)
//! 0x1C 4     aux_off: byte offset past the channel wave infos
//! 0x20 4     reserved (0 seen)
//! 0x24 4     channels
//! 0x28 4     flags (0 seen; 2 in one long music file that carries extra data)
//! 0x2C 4     data_off: byte offset of block 0 (file size = data_off +
//!            blocks * block_chunk)
//! ```
//!
//! The channel region at `ch_table_off` holds a preface of one 16-byte entry
//! per channel (`info_off`, a cumulative byte offset; `name_hash`; and the
//! per-channel `record_len`), followed by one wave info per channel, each
//! the 32-byte bank wave header plus a trailer. The trailer length in bytes
//! is a 21- or 24-byte header plus `3 * entries` where `entries` is the
//! channel's total seek entries across all blocks (the trailer contents and
//! what selects 24 are unknown). Bytes between the channel infos (`aux_off`)
//! and the file
//! table exist only when the flags word is non-zero and look like timestamp
//! quads; they are exposed raw.
//!
//! The file table holds `blocks` entries of 8 bytes: the block's first sample
//! index (u32) and the sample rate (u32).
//!
//! Each block starts with three u64 words: the base header size (24),
//! the channel-info end / seek-table start offset (56), and the seek-table
//! start offset (56). Then come `channels` channel infos of 16 bytes:
//! `start_entry`, `entries`, `skip` (0 except in 52 shipped PCM files, where it
//! is a signed drift marker the decoder ignores; see
//! [`streamed::BlockChannel::skip`]), `sample_count`. Then the seek
//! tables: per channel, `entries` pairs of u32 `(first_sample, last_sample)`
//! with absolute sample indices; each entry covers 2048 ADPCM bytes (4096
//! samples). Payload starts at the block start plus the seek-table end
//! rounded up to 2048; channel `c` owns bytes
//! `payload + start_entry * 2048` for `entries * 2048` bytes. Channels are
//! stored whole (not interleaved) and the ADPCM state carries across block
//! boundaries during linear playback; only a mid-stream seek starts a block
//! from a fresh state.
//!
//! ## RPF3 audio archives
//!
//! The `.rpf` files under `pc/audio/sfx` hold the bank and streamed payloads
//! this crate parses. They are opened with the `lf-archive` crate, which
//! reads the table of contents (decrypting it with the key located at run
//! time in the owner's executable) and yields each entry's raw bytes; see
//! that crate's documentation for the archive layout.
//! ## Versioned audio configs
//!
//! `effects.dat11`, `curves.dat12`, `categories.dat15`, `sounds.dat15` and
//! `game.dat16` share one outer shape: `version` (u32, equals the number in
//! the extension), `names_off` (u32, byte offset where the name region
//! starts), and two more u32 header words of unknown meaning. Bytes from the
//! 16-byte header to `names_off` are the object region (opaque); from
//! `names_off` on are length-prefixed (`u8` length + bytes) names with
//! per-name metadata words whose split is still unknown. `speech.dat` has no
//! version suffix and a different layout that is not understood yet.
//!
//! ## What is still unknown
//!
//! Bank/stream record trailers, the streamed timestamp region, the two
//! unknown bank header words, config object records and per-name metadata,
//! and the whole `speech.dat` layout. Unknown bytes are always exposed raw so
//! a later tool can re-read them without re-parsing.

#![forbid(unsafe_code)]

pub mod adpcm;
pub mod bank;
pub mod dat;
pub mod streamed;

use std::fmt;

/// What kind of parse failure happened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// The input ended before the parse finished.
    Truncated,
    /// A magic word or tag did not match.
    BadMagic,
    /// The input is well-formed but uses an unsupported variant.
    Unsupported,
    /// A field value is inconsistent with the rest of the input.
    Invalid,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Truncated => write!(f, "truncated"),
            ErrorKind::BadMagic => write!(f, "bad magic"),
            ErrorKind::Unsupported => write!(f, "unsupported"),
            ErrorKind::Invalid => write!(f, "invalid"),
        }
    }
}

/// A parse failure: where it happened and what went wrong.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    offset: u64,
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub(crate) fn new(offset: u64, kind: ErrorKind, message: String) -> Error {
        Error {
            offset,
            kind,
            message,
        }
    }

    /// Byte offset in the parsed input where the failure was detected.
    #[must_use]
    pub fn offset(&self) -> u64 {
        self.offset
    }

    /// The kind of failure.
    #[must_use]
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Human-readable detail.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at offset {}: {}",
            self.kind, self.offset, self.message
        )
    }
}

impl std::error::Error for Error {}

/// Little-endian cursor over a borrowed slice. Every read is bounds-checked.
#[derive(Debug, Clone)]
pub(crate) struct Cursor<'a> {
    buf: &'a [u8],
    pos: u64,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(buf: &'a [u8]) -> Cursor<'a> {
        Cursor { buf, pos: 0 }
    }

    pub(crate) fn len(&self) -> u64 {
        u64::try_from(self.buf.len()).unwrap_or(u64::MAX)
    }

    pub(crate) fn pos(&self) -> u64 {
        self.pos
    }

    pub(crate) fn seek(&mut self, pos: u64) -> Result<(), Error> {
        if pos > self.len() {
            return Err(Error::new(
                self.pos,
                ErrorKind::Truncated,
                format!("seek to {pos} past end {}", self.len()),
            ));
        }
        self.pos = pos;
        Ok(())
    }

    pub(crate) fn bytes(&mut self, n: usize) -> Result<&'a [u8], Error> {
        let start = self.pos;
        let want = u64::try_from(n).unwrap_or(u64::MAX);
        let end = start
            .checked_add(want)
            .filter(|&e| e <= self.len())
            .ok_or_else(|| {
                Error::new(
                    start,
                    ErrorKind::Truncated,
                    format!("need {n} bytes at {start}, input is {}", self.len()),
                )
            })?;
        self.pos = end;
        // `end` was checked against the input length, so this is always `Some`.
        crate::slice_u64(self.buf, start, end).ok_or_else(|| {
            Error::new(
                start,
                ErrorKind::Truncated,
                format!("need {n} bytes at {start}, input is {}", self.len()),
            )
        })
    }

    pub(crate) fn u16(&mut self) -> Result<u16, Error> {
        let b = self.bytes(2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    pub(crate) fn u32(&mut self) -> Result<u32, Error> {
        let b = self.bytes(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    pub(crate) fn u64(&mut self) -> Result<u64, Error> {
        let b = self.bytes(8)?;
        Ok(u64::from_le_bytes([
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
        ]))
    }
}

/// Round `v` up to a multiple of `align` (which must be a power of two).
pub(crate) fn align_up(v: u64, align: u64) -> u64 {
    v.saturating_add(align - 1) & !(align - 1)
}

/// Borrow `buf[start..end]` with fallible index conversion. Callers validate
/// the range against the input length first; `None` is unreachable then, and
/// every caller maps it to a truncation error.
pub(crate) fn slice_u64(buf: &[u8], start: u64, end: u64) -> Option<&[u8]> {
    let s = usize::try_from(start).ok()?;
    let e = usize::try_from(end).ok()?;
    buf.get(s..e)
}

/// Copy `sample_count` little-endian 16-bit PCM samples from the front of
/// `data`. Used for the `0x1` codec in both containers.
pub(crate) fn decode_pcm16(data: &[u8], sample_count: usize, at: u64) -> Result<Vec<i16>, Error> {
    let need = sample_count
        .checked_mul(2)
        .ok_or_else(|| Error::new(at, ErrorKind::Invalid, "sample count overflows".to_string()))?;
    if data.len() < need {
        return Err(Error::new(
            at,
            ErrorKind::Truncated,
            format!("need {need} PCM bytes, input is {}", data.len()),
        ));
    }
    Ok(data[..need]
        .chunks_exact(2)
        .map(|c| i16::from_le_bytes([c[0], c[1]]))
        .collect())
}

/// A parsed audio container: either a multi-sound [`bank::Bank`] or a
/// single-sound [`streamed::Streamed`].
#[derive(Debug)]
pub enum Container<'a> {
    /// Multi-sound bank container.
    Bank(bank::Bank<'a>),
    /// Single-sound streamed (blocked) container.
    Streamed(streamed::Streamed<'a>),
}

/// Detect which container `buf` holds and parse it.
///
/// The word at offset 0x10 decides: banks store their stream count there
/// (always >= 1) while streamed files store 0. Inputs shorter than 28 bytes
/// fail as truncated.
///
/// # Errors
///
/// Returns an error when the input is shorter than 28 bytes or the
/// detected container fails to parse.
pub fn detect(buf: &[u8]) -> Result<Container<'_>, Error> {
    if Cursor::new(buf).len() < 28 {
        return Err(Error::new(
            0,
            ErrorKind::Truncated,
            format!("need 28 bytes to detect, input is {}", buf.len()),
        ));
    }
    let mut cur = Cursor::new(buf);
    cur.seek(0x10)?;
    let count_or_zero = cur.u32()?;
    if count_or_zero == 0 {
        Ok(Container::Streamed(streamed::Streamed::parse(buf)?))
    } else {
        Ok(Container::Bank(bank::Bank::parse(buf)?))
    }
}
