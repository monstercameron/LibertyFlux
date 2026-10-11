//! Multi-sound bank containers.
//!
//! A bank holds any number of short sounds. The 28-byte header points at a
//! stream table; each table entry points at a packed info record (a 32-byte
//! wave header plus a trailer of unknown contents); the wave header points
//! at ADPCM bytes. See the crate documentation for the layout tables.

use crate::adpcm;
use crate::{Cursor, Error, ErrorKind, decode_pcm16};

/// Codec tag for IMA ADPCM decoded in software.
pub const CODEC_ADPCM: u32 = 0x400;
/// Codec tag for 16-bit little-endian PCM (GPS voices, some radio).
pub const CODEC_PCM16: u32 = 0x1;

/// Size of the bank header and of one stream-table entry, in bytes.
pub const HEADER_LEN: u64 = 28;
/// Size of one stream-table entry, in bytes.
pub const TABLE_ENTRY_LEN: u64 = 16;
/// Size of the wave header at the start of each info record, in bytes.
pub const WAVE_HEADER_LEN: u64 = 32;

/// A parsed bank container borrowing its input.
#[derive(Debug)]
pub struct Bank<'a> {
    buf: &'a [u8],
    table_off: u64,
    records_end: u32,
    count: u32,
    reserved_0c: u32,
    field_14: u32,
    base: u32,
}

/// One entry of the stream table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BankEntry {
    /// Index in the table (vgmstream subsong numbers are this plus one).
    pub index: u32,
    /// Byte offset of the stream's info record, relative to the end of the
    /// table. Records are packed, so this is not a multiple of anything.
    pub info_off: u64,
    /// Name hash of the stream.
    pub name_hash: u32,
    /// Length in bytes of the info record including its trailer.
    pub record_len: u32,
}

/// A stream's info record: the wave header plus its raw trailer.
#[derive(Debug)]
pub struct StreamInfo<'a> {
    /// The table entry this record was reached through.
    pub entry: BankEntry,
    /// Byte offset of the sound's ADPCM bytes, relative to [`Bank::base`].
    pub data_off: u64,
    /// ADPCM bytes; equals `sample_count / 2` rounded down.
    pub size: u32,
    /// Total samples in the sound.
    pub sample_count: u32,
    /// Flags word; 0 or all-ones in files seen, meaning unknown.
    pub flags_14: u32,
    /// Sample rate in Hz.
    pub sample_rate: u16,
    /// Unknown word.
    pub field_1a: u16,
    /// Codec tag ([`CODEC_ADPCM`] in every file seen).
    pub codec: u32,
    /// The `record_len - 32` bytes after the wave header, contents unknown.
    pub trailer: &'a [u8],
}

impl<'a> Bank<'a> {
    /// Parse a bank container from the whole file contents.
    ///
    /// # Errors
    ///
    /// Returns an error when the input is shorter than the 28-byte header,
    /// when the table or data offsets run past the input, or when the
    /// stream count is zero.
    pub fn parse(buf: &'a [u8]) -> Result<Bank<'a>, Error> {
        let mut cur = Cursor::new(buf);
        let table_off = cur.u64()?;
        let records_end = cur.u32()?;
        let reserved_0c = cur.u32()?;
        let count = cur.u32()?;
        let field_14 = cur.u32()?;
        let base = cur.u32()?;
        if table_off < HEADER_LEN {
            return Err(Error::new(
                0,
                ErrorKind::Invalid,
                format!("table offset {table_off} inside the 28-byte header"),
            ));
        }
        if count == 0 {
            return Err(Error::new(
                0x10,
                ErrorKind::Invalid,
                "bank with zero streams (streamed files use the streamed parser)".to_string(),
            ));
        }
        let table_end = table_off
            .checked_add(u64::from(count) * TABLE_ENTRY_LEN)
            .ok_or_else(|| {
                Error::new(
                    0x10,
                    ErrorKind::Invalid,
                    "stream count overflows".to_string(),
                )
            })?;
        if table_end > buf.len() as u64 {
            return Err(Error::new(
                table_off,
                ErrorKind::Truncated,
                format!(
                    "table of {count} entries ends at {table_end}, input is {}",
                    buf.len()
                ),
            ));
        }
        if u64::from(records_end) > buf.len() as u64 {
            return Err(Error::new(
                0x08,
                ErrorKind::Invalid,
                format!("records end {}, input is {}", records_end, buf.len()),
            ));
        }
        if u64::from(base) > buf.len() as u64 {
            return Err(Error::new(
                0x18,
                ErrorKind::Invalid,
                format!("data base {base}, input is {}", buf.len()),
            ));
        }
        Ok(Bank {
            buf,
            table_off,
            records_end,
            count,
            reserved_0c,
            field_14,
            base,
        })
    }

    /// Byte offset of the stream table.
    #[must_use]
    pub fn table_off(&self) -> u64 {
        self.table_off
    }

    /// Byte offset where the packed info records end.
    #[must_use]
    pub fn records_end(&self) -> u32 {
        self.records_end
    }

    /// Number of streams.
    #[must_use]
    pub fn stream_count(&self) -> u32 {
        self.count
    }

    /// Reserved header word at 0x0C (0 in every file seen).
    #[must_use]
    pub fn reserved_0c(&self) -> u32 {
        self.reserved_0c
    }

    /// Unknown header word at 0x14.
    #[must_use]
    pub fn field_14(&self) -> u32 {
        self.field_14
    }

    /// Byte offset where ADPCM data starts; stream data offsets are relative
    /// to this.
    #[must_use]
    pub fn base(&self) -> u32 {
        self.base
    }

    /// Byte offset of the first info record (the end of the stream table).
    #[must_use]
    pub fn records_start(&self) -> u64 {
        self.table_off + u64::from(self.count) * TABLE_ENTRY_LEN
    }

    /// The zero padding between the info records and the ADPCM data.
    ///
    /// Empty when a corrupt header puts the records end past the data base
    /// (found by the mutation fuzzer; slicing used to panic).
    #[must_use]
    pub fn gap(&self) -> &'a [u8] {
        self.buf
            .get(self.records_end as usize..self.base as usize)
            .unwrap_or(&[])
    }

    /// Read the whole stream table.
    ///
    /// # Errors
    ///
    /// Returns an error when the table runs past the end of the input.
    pub fn entries(&self) -> Result<Vec<BankEntry>, Error> {
        let mut cur = Cursor::new(self.buf);
        cur.seek(self.table_off)?;
        let mut out = Vec::with_capacity(self.count as usize);
        for index in 0..self.count {
            let info_off = cur.u64()?;
            let name_hash = cur.u32()?;
            let record_len = cur.u32()?;
            out.push(BankEntry {
                index,
                info_off,
                name_hash,
                record_len,
            });
        }
        Ok(out)
    }

    /// Read one stream's info record through its table entry.
    ///
    /// # Errors
    ///
    /// Returns an error when the record length is below the 32-byte wave
    /// header, when the record runs past the input, or when its hash
    /// disagrees with the table entry.
    pub fn stream(&self, entry: &BankEntry) -> Result<StreamInfo<'a>, Error> {
        if u64::from(entry.record_len) < WAVE_HEADER_LEN {
            return Err(Error::new(
                self.table_off + u64::from(entry.index) * TABLE_ENTRY_LEN,
                ErrorKind::Invalid,
                format!(
                    "record length {} below wave header size 32",
                    entry.record_len
                ),
            ));
        }
        let records_start = self.records_start();
        let at = records_start.checked_add(entry.info_off).ok_or_else(|| {
            Error::new(
                records_start,
                ErrorKind::Invalid,
                format!("info offset {} overflows", entry.info_off),
            )
        })?;
        let mut cur = Cursor::new(self.buf);
        cur.seek(at)?;
        let data_off = cur.u64()?;
        let name_hash = cur.u32()?;
        let size = cur.u32()?;
        let sample_count = cur.u32()?;
        let flags_14 = cur.u32()?;
        let sample_rate = cur.u16()?;
        let field_1a = cur.u16()?;
        let codec = cur.u32()?;
        // `bytes` re-checks the length against the input, so an absurd
        // record length fails closed here even on 32-bit targets.
        let trailer_len =
            usize::try_from(u64::from(entry.record_len) - WAVE_HEADER_LEN).unwrap_or(usize::MAX);
        let trailer = cur.bytes(trailer_len)?;
        if name_hash != entry.name_hash {
            return Err(Error::new(
                at + 8,
                ErrorKind::Invalid,
                format!(
                    "record hash {name_hash:#x} disagrees with table hash {:#x}",
                    entry.name_hash
                ),
            ));
        }
        Ok(StreamInfo {
            entry: *entry,
            data_off,
            size,
            sample_count,
            flags_14,
            sample_rate,
            field_1a,
            codec,
            trailer,
        })
    }

    /// Borrow one stream's ADPCM bytes.
    ///
    /// # Errors
    ///
    /// Returns an error when the data range overflows or runs past the
    /// end of the input.
    pub fn stream_data(&self, stream: &StreamInfo<'_>) -> Result<&'a [u8], Error> {
        let base = u64::from(self.base);
        let start = base.checked_add(stream.data_off).ok_or_else(|| {
            Error::new(
                base,
                ErrorKind::Invalid,
                format!("data offset {} overflows", stream.data_off),
            )
        })?;
        let end = start.checked_add(u64::from(stream.size)).ok_or_else(|| {
            Error::new(
                start,
                ErrorKind::Invalid,
                format!("stream size {} overflows", stream.size),
            )
        })?;
        let len = u64::try_from(self.buf.len()).unwrap_or(u64::MAX);
        if end > len {
            return Err(Error::new(
                start,
                ErrorKind::Truncated,
                format!(
                    "stream {} data ends at {end}, input is {}",
                    stream.entry.index,
                    self.buf.len()
                ),
            ));
        }
        crate::slice_u64(self.buf, start, end).ok_or_else(|| {
            Error::new(
                start,
                ErrorKind::Truncated,
                format!("stream {} data out of range", stream.entry.index),
            )
        })
    }

    /// Decode one stream to mono 16-bit PCM. [`CODEC_ADPCM`] and
    /// [`CODEC_PCM16`] are supported; anything else (such as the lone Vorbis
    /// stream's `0x200`) fails as unsupported.
    ///
    /// # Errors
    ///
    /// Returns an error when the sound's data runs past the input or its
    /// codec has no decoder.
    pub fn decode(&self, stream: &StreamInfo<'_>) -> Result<Vec<i16>, Error> {
        let at = self
            .records_start()
            .saturating_add(stream.entry.info_off)
            .saturating_add(0x1c);
        match stream.codec {
            CODEC_ADPCM => {
                let data = self.stream_data(stream)?;
                let want = stream.sample_count as usize;
                if want == data.len() * 2 + 1 {
                    // Odd sample counts store size = samples / 2 rounded
                    // down, so the final nibble lies one nibble past the
                    // data in the zero sector padding. Append the zero byte
                    // explicitly instead of reading past the stream's bytes.
                    let mut padded = Vec::with_capacity(data.len() + 1);
                    padded.extend_from_slice(data);
                    padded.push(0);
                    return adpcm::decode_mono(&padded, want);
                }
                adpcm::decode_mono(data, want)
            }
            CODEC_PCM16 => {
                let data = self.stream_data(stream)?;
                decode_pcm16(data, stream.sample_count as usize, at)
            }
            codec => Err(Error::new(
                at,
                ErrorKind::Unsupported,
                format!("codec {codec:#x} has no decoder"),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a one-stream bank: 28-byte header, one 16-byte table entry, one
    /// 40-byte record (32-byte wave header + 8-byte trailer), 4 data bytes.
    fn one_stream_bank() -> Vec<u8> {
        let mut b = vec![0u8; 28 + 16 + 40 + 4 + 4];
        // header: table=28, records_end=28+16+40=84, count=1, base=88
        b[0..8].copy_from_slice(&28u64.to_le_bytes());
        b[8..12].copy_from_slice(&84u32.to_le_bytes());
        b[16..20].copy_from_slice(&1u32.to_le_bytes());
        b[24..28].copy_from_slice(&88u32.to_le_bytes());
        // entry: info_off=0, hash=0xAABBCCDD, record_len=40
        b[28..36].copy_from_slice(&0u64.to_le_bytes());
        b[36..40].copy_from_slice(&0xAABB_CCDDu32.to_le_bytes());
        b[40..44].copy_from_slice(&40u32.to_le_bytes());
        // record at 44: data_off=0, hash, size=4, samples=8, rate=22050, codec
        b[44..52].copy_from_slice(&0u64.to_le_bytes());
        b[52..56].copy_from_slice(&0xAABB_CCDDu32.to_le_bytes());
        b[56..60].copy_from_slice(&4u32.to_le_bytes());
        b[60..64].copy_from_slice(&8u32.to_le_bytes());
        b[68..70].copy_from_slice(&22_050u16.to_le_bytes());
        b[72..76].copy_from_slice(&CODEC_ADPCM.to_le_bytes());
        b[76..80].copy_from_slice(&56u32.to_le_bytes()); // trailer opens with 56
        b[80..84].copy_from_slice(&7u32.to_le_bytes());
        // gap 84..88 stays zero; data 88..92
        b[88..92].copy_from_slice(&[0x77, 0x77, 0x00, 0x00]);
        b
    }

    #[test]
    fn parses_hand_built_bank() {
        let b = one_stream_bank();
        let bank = Bank::parse(&b).unwrap();
        assert_eq!(bank.stream_count(), 1);
        assert_eq!(bank.base(), 88);
        assert_eq!(bank.records_start(), 44);
        assert_eq!(bank.gap(), &[0, 0, 0, 0]);
        let entries = bank.entries().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name_hash, 0xAABB_CCDD);
        let s = bank.stream(&entries[0]).unwrap();
        assert_eq!(s.sample_rate, 22_050);
        assert_eq!(s.sample_count, 8);
        assert_eq!(s.size, 4);
        assert_eq!(s.trailer.len(), 8);
        assert_eq!(bank.stream_data(&s).unwrap(), &[0x77, 0x77, 0x00, 0x00]);
        // First four samples are the hand-computed ramp; the zero nibbles
        // then climb by step>>3 each (19, 17, 16, 14) from predictor 240.
        assert_eq!(
            bank.decode(&s).unwrap(),
            vec![11, 41, 104, 240, 259, 276, 292, 306]
        );
    }

    #[test]
    fn rejects_short_and_inconsistent_banks() {
        assert_eq!(
            Bank::parse(&[0u8; 27]).unwrap_err().kind(),
            ErrorKind::Truncated
        );
        let mut b = one_stream_bank();
        b[0] = 4; // table inside the header
        assert_eq!(Bank::parse(&b).unwrap_err().kind(), ErrorKind::Invalid);
        let mut b = one_stream_bank();
        b[52] ^= 0xff; // record hash disagrees with table hash
        let bank = Bank::parse(&b).unwrap();
        let e = bank.entries().unwrap();
        assert_eq!(bank.stream(&e[0]).unwrap_err().kind(), ErrorKind::Invalid);
    }

    #[test]
    fn odd_sample_count_reads_zero_padding() {
        // 7 samples in 3 bytes: the last nibble comes from zero padding.
        let mut b = one_stream_bank();
        b[56..60].copy_from_slice(&3u32.to_le_bytes());
        b[60..64].copy_from_slice(&7u32.to_le_bytes());
        let bank = Bank::parse(&b).unwrap();
        let e = bank.entries().unwrap();
        let s = bank.stream(&e[0]).unwrap();
        assert_eq!(
            bank.decode(&s).unwrap(),
            vec![11, 41, 104, 240, 259, 276, 292]
        );
    }

    #[test]
    fn refuses_unknown_codec() {
        let mut b = one_stream_bank();
        b[72..76].copy_from_slice(&0x100u32.to_le_bytes());
        let bank = Bank::parse(&b).unwrap();
        let e = bank.entries().unwrap();
        let s = bank.stream(&e[0]).unwrap();
        assert_eq!(bank.decode(&s).unwrap_err().kind(), ErrorKind::Unsupported);
    }

    #[test]
    fn pcm_decode_copies_samples() {
        let mut b = one_stream_bank();
        // Rework the stream as 2 PCM samples in 4 bytes.
        b[56..60].copy_from_slice(&4u32.to_le_bytes());
        b[60..64].copy_from_slice(&2u32.to_le_bytes());
        b[72..76].copy_from_slice(&CODEC_PCM16.to_le_bytes());
        b[88..92].copy_from_slice(&[0x34, 0x12, 0xFF, 0xFF]);
        let bank = Bank::parse(&b).unwrap();
        let e = bank.entries().unwrap();
        let s = bank.stream(&e[0]).unwrap();
        assert_eq!(bank.decode(&s).unwrap(), vec![0x1234, -1]);
    }
}
