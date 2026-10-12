//! Counted runs: a search for one word among the entries of a counted run of
//! words that sits inside an object.
//!
//! Several small routines of the original look for one 32-bit value in a run
//! of entries: the run starts at a fixed offset inside an object, its entries
//! are a fixed distance apart, and the number of entries is either fixed in
//! the routine or read from a word of the object. Each routine compares the
//! word at the start of each entry with the value it is given, in order, and
//! stops at the first match. The routines differ in where the count comes
//! from, where the run starts, the entry size, whether a zero value is ever a
//! match, and what they return. [`CountedRun`] holds one such routine as data.
//! [`CountedRun::evaluate`] is the routine. The instances the verified
//! rewrites were checked against are listed in the test crate that compares
//! them.
//!
//! The object is seen as its bytes, little endian, starting at offset zero.
//! The offsets below are offsets from the object's start. The routine reads
//! the bytes it needs and nothing else; it writes nothing.
//!
//! Specification of [`CountedRun::evaluate`]:
//!
//! - When [`CountedRun::zero_key_misses`] is set and the key is zero, the
//!   answer is a miss at once, and the object is not read.
//! - Otherwise the count is read first. [`Count::Fixed`] is the number given.
//!   [`Count::Unsigned`] and [`Count::Signed`] read a 32-bit word of the
//!   object at their offset; a signed count of zero or less holds no entries.
//! - Entry `i` (counting from zero) starts at the first offset plus `i` times
//!   the stride, in 32-bit wrapping arithmetic. The word at its start is
//!   compared with the key. Entries are compared in order, and the first
//!   equal one is the match.
//! - A match gives entry index `i` and that entry's offset. No match gives the
//!   offset just past the run, the first offset plus the count times the
//!   stride, again in wrapping arithmetic.
//! - The answer is then one of four forms ([`Answer`]). A flag answers 1 for
//!   a match and 0 for no match. An index answers the match's index, or
//!   `0xFFFF_FFFF` for no match. A masked entry address answers the matched
//!   entry's address with its low byte cleared and bit 0 set, or the address
//!   just past the run with its low byte cleared; it needs the object's
//!   address, which the caller passes in. A masked key answers the key with
//!   its low byte cleared, and bit 0 set for a match.
//! - A run that reaches past the end of the bytes given is an error
//!   ([`Truncated`]), reported at the first read that falls outside them. The
//!   original would read that memory; a caller that gives the whole object
//!   never sees the error.
//! - The loop runs at most as many times as the count, and each pass reads one
//!   entry of the given bytes, so a run that ends outside the bytes stops it.

/// How many entries a run holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Count {
    /// A count that the routine fixes.
    Fixed(u32),
    /// An unsigned 32-bit word of the object, read at this offset.
    Unsigned {
        /// Byte offset of the count word.
        offset: u32,
    },
    /// A signed 32-bit word of the object, read at this offset; zero and
    /// negative counts hold no entries.
    Signed {
        /// Byte offset of the count word.
        offset: u32,
    },
}

/// What a search answers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Answer {
    /// 1 for a match, 0 for no match.
    Flag,
    /// The index of the match, or `0xFFFF_FFFF` for no match.
    Index,
    /// The matched entry's address with its low byte cleared and bit 0 set,
    /// or the address just past the run with its low byte cleared.
    MaskedEntryAddress,
    /// The key with its low byte cleared and bit 0 set for a match, or with
    /// its low byte cleared for no match.
    MaskedKey,
}

/// The index an [`Answer::Index`] gives for no match.
pub const NO_INDEX: u32 = 0xFFFF_FFFF;

/// The object is too short for the run: a read fell past its last byte.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Truncated;

/// One counted-run search: where the run is, how it is counted and what the
/// search answers. Plain data; the search itself is [`CountedRun::evaluate`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CountedRun {
    /// Where the number of entries comes from.
    pub count: Count,
    /// Byte offset of entry zero from the object's start.
    pub first: u32,
    /// Byte distance from one entry's start to the next.
    pub stride: u32,
    /// When set, a zero key is a miss without reading the object.
    pub zero_key_misses: bool,
    /// The form of the answer.
    pub answer: Answer,
}

/// The outcome of the scan, before the answer's form is applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scan {
    /// Entry `index`, which starts at `offset`, is the first equal one.
    Hit { index: u32, offset: u32 },
    /// No entry is equal; `end` is the offset just past the run.
    Miss { end: u32 },
    /// The key is zero and zero keys never match: the object was not read.
    Guarded,
}

/// Reads the 32-bit little-endian word at `offset`, or reports truncation.
fn word_at(object: &[u8], offset: u32) -> Result<u32, Truncated> {
    let start = offset as usize;
    let bytes = object.get(start..start + 4).ok_or(Truncated)?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

impl CountedRun {
    /// The number of entries the run holds, as the original reads it.
    fn entry_count(&self, object: &[u8]) -> Result<u32, Truncated> {
        match self.count {
            Count::Fixed(n) => Ok(n),
            Count::Unsigned { offset } => word_at(object, offset),
            Count::Signed { offset } => {
                let raw = word_at(object, offset)? as i32;
                Ok(if raw > 0 { raw as u32 } else { 0 })
            }
        }
    }

    /// The offset just past the run for a count of `count` entries.
    fn end_of(&self, count: u32) -> u32 {
        self.first.wrapping_add(count.wrapping_mul(self.stride))
    }

    /// Compares the entries in order with `key` and reports the first match.
    fn scan(&self, object: &[u8], key: u32) -> Result<Scan, Truncated> {
        if self.zero_key_misses && key == 0 {
            return Ok(Scan::Guarded);
        }
        let count = self.entry_count(object)?;
        let mut index = 0u32;
        while index < count {
            let offset = self.first.wrapping_add(index.wrapping_mul(self.stride));
            if word_at(object, offset)? == key {
                return Ok(Scan::Hit { index, offset });
            }
            index += 1;
        }
        Ok(Scan::Miss {
            end: self.end_of(count),
        })
    }

    /// Searches the run of `object` for `key` and gives the answer in this
    /// run's form. `object_address` is the address the object sits at in the
    /// caller's space; it is used only by [`Answer::MaskedEntryAddress`].
    ///
    /// # Errors
    ///
    /// [`Truncated`] when a read of the run falls past the end of `object`.
    pub fn evaluate(&self, object: &[u8], object_address: u32, key: u32) -> Result<u32, Truncated> {
        let scan = self.scan(object, key)?;
        let answer = match (self.answer, scan) {
            (Answer::Flag, Scan::Hit { .. }) => 1,
            (Answer::Flag, Scan::Miss { .. } | Scan::Guarded) => 0,
            (Answer::Index, Scan::Hit { index, .. }) => index,
            (Answer::Index, Scan::Miss { .. } | Scan::Guarded) => NO_INDEX,
            (Answer::MaskedEntryAddress, Scan::Hit { offset, .. }) => {
                (object_address.wrapping_add(offset) & !0xFF) | 1
            }
            (Answer::MaskedEntryAddress, Scan::Miss { end }) => {
                object_address.wrapping_add(end) & !0xFF
            }
            (Answer::MaskedEntryAddress, Scan::Guarded) => {
                // The run's end is still defined for a guarded key: the count
                // is read as it is for any other search.
                let end = self.end_of(self.entry_count(object)?);
                object_address.wrapping_add(end) & !0xFF
            }
            (Answer::MaskedKey, Scan::Hit { .. }) => (key & !0xFF) | 1,
            (Answer::MaskedKey, Scan::Miss { .. } | Scan::Guarded) => key & !0xFF,
        };
        Ok(answer)
    }
}
