//! Sentinel tables: a run of fixed-size records, each one an empty marker word
//! and a cleared half-word, cleared all at once, with an optional flag byte
//! set just before the run.
//!
//! Several small routines of the original reset a table of records to the
//! empty state. Each record is a 32-bit word that holds the empty marker
//! `0xFFFF_FFFF` at the record's start, and a 16-bit word at four bytes into
//! the record that is set to zero. The bytes after those six are left alone.
//! The run has a fixed number of records and a fixed record size. Some of the
//! routines also clear a flag byte that sits a fixed distance before the run,
//! after the records. Each routine returns the address just past the run.
//! [`SentinelTable`] holds one such routine as data, and
//! [`SentinelTable::fill`] is the routine.
//!
//! The table is seen as the bytes of an image, little endian. The run's start
//! is an index into that image; the flag, when there is one, is at the index
//! the offset gives from the run's start. Nothing here reads a byte, and
//! nothing here depends on an address.
//!
//! Specification of [`SentinelTable::fill`]:
//!
//! - Record `i` (counting from zero) starts at the run's start plus `i` times
//!   the record size. The routine writes [`EMPTY`] as a 32-bit little-endian
//!   word at the record's start, then zero as a 16-bit word four bytes in. It
//!   writes nothing else in the record.
//! - Records are cleared in order, from the first to the last.
//! - When the table has a flag, the routine then writes zero to the flag byte,
//!   at the run's start plus the flag offset.
//! - The result is the index just past the run: the run's start plus the
//!   number of records times the record size.
//! - A write that falls outside the image is an error ([`Fault`]). The
//!   original would write that memory; a caller that gives the whole image
//!   never sees the error. The writes made before the fault are kept.
//! - The loop runs once per record, and each pass writes inside the given
//!   image, so the loop is bounded by the record count.

/// The empty marker a cleared record holds in its first word.
pub const EMPTY: u32 = 0xFFFF_FFFF;

/// Bytes into a record at which the cleared half-word starts.
pub const HALF_OFFSET: usize = 4;

/// Bytes a record occupies that the routine writes: the marker and the half-word.
pub const WRITTEN_BYTES: usize = 6;

/// A write fell outside the image, or an index overflowed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fault;

/// One sentinel-table reset: the record count, the record size and the
/// optional flag byte's offset from the run's start. Plain data; the routine
/// is [`SentinelTable::fill`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SentinelTable {
    /// The number of records in the run.
    pub records: u32,
    /// The distance in bytes from one record's start to the next.
    pub stride: u32,
    /// The flag byte's offset from the run's start (negative for a flag before
    /// the run), when the routine clears one.
    pub flag: Option<i32>,
}

impl SentinelTable {
    /// Clears the run that starts at `table` in `image`, and the flag byte if
    /// the table has one. Returns the index just past the run.
    ///
    /// # Errors
    ///
    /// [`Fault`] when a write falls outside `image` or an index overflows.
    pub fn fill(&self, image: &mut [u8], table: usize) -> Result<usize, Fault> {
        let stride = self.stride as usize;
        let mut record = 0u32;
        while record < self.records {
            let start = table
                .checked_add((record as usize).checked_mul(stride).ok_or(Fault)?)
                .ok_or(Fault)?;
            let end = start.checked_add(WRITTEN_BYTES).ok_or(Fault)?;
            let bytes = image.get_mut(start..end).ok_or(Fault)?;
            bytes[..4].copy_from_slice(&EMPTY.to_le_bytes());
            bytes[HALF_OFFSET..WRITTEN_BYTES].copy_from_slice(&0u16.to_le_bytes());
            record += 1;
        }
        let past = table
            .checked_add((self.records as usize).checked_mul(stride).ok_or(Fault)?)
            .ok_or(Fault)?;
        if let Some(offset) = self.flag {
            let index = table.checked_add_signed(offset as isize).ok_or(Fault)?;
            *image.get_mut(index).ok_or(Fault)? = 0;
        }
        Ok(past)
    }
}
