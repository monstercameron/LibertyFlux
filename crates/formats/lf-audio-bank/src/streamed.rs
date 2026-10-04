//! Single-sound streamed (blocked) containers.
//!
//! A streamed file holds one long sound split into `blocks` blocks of
//! `block_chunk` bytes. Each block carries its own channel infos and seek
//! tables; linear playback decodes with ADPCM state carried across block
//! boundaries. See the crate documentation for the layout tables.

use crate::adpcm::ImaState;
use crate::bank::{CODEC_ADPCM, CODEC_PCM16};
use crate::{Cursor, Error, ErrorKind, align_up, decode_pcm16};

/// Size of the streamed file header, in bytes.
pub const HEADER_LEN: u64 = 48;
/// Size of one per-block file-table entry, in bytes.
pub const FILE_TABLE_ENTRY_LEN: u64 = 8;
/// Size of one channel's preface entry, in bytes; the preface holds one per
/// channel.
pub const PREFACE_ENTRY_LEN: u64 = 16;
/// Size of one per-block channel info, in bytes.
pub const BLOCK_CH_INFO_LEN: u64 = 16;
/// ADPCM bytes covered by one seek entry (4096 samples).
pub const SEEK_ENTRY_BYTES: u64 = 2048;
/// Policy cap on one channel's decoded length: 2^28 samples (512 MiB of
/// 16-bit PCM). The longest shipped channel holds far fewer; anything above
/// is a hostile or corrupt header, refused rather than allocated.
pub const MAX_DECODED_SAMPLES: u32 = 1 << 28;

/// A parsed streamed container borrowing its input.
#[derive(Debug)]
pub struct Streamed<'a> {
    buf: &'a [u8],
    table_off: u64,
    blocks: u32,
    block_chunk: u32,
    ch_table_off: u64,
    aux_off: u32,
    reserved_20: u32,
    channels: u32,
    flags_28: u32,
    data_off: u32,
}

/// One channel's entry in the channel-region preface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrefaceChannel {
    /// Channel index.
    pub index: u32,
    /// Cumulative byte offset of this channel's wave info within the
    /// channel-info run (channel index times the record length).
    pub info_off: u64,
    /// Name hash of the channel.
    pub name_hash: u32,
    /// Length in bytes of one channel's wave info including its trailer.
    pub record_len: u32,
}

/// One channel's wave header plus its raw trailer.
#[derive(Debug)]
pub struct ChannelWave<'a> {
    /// Channel index.
    pub index: u32,
    /// Data-offset word from the shared wave header shape (0 in files seen;
    /// streamed payload is located through block seek tables instead).
    pub data_off: u64,
    /// Name hash.
    pub name_hash: u32,
    /// ADPCM bytes for this channel; equals `sample_count / 2` rounded down.
    pub size: u32,
    /// Total samples in this channel.
    pub sample_count: u32,
    /// Flags word; meaning unknown.
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

/// One entry of the per-block file table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockTableEntry {
    /// Absolute index of the block's first sample.
    pub first_sample: u32,
    /// Sample rate in Hz (repeats the channel headers).
    pub sample_rate: u32,
}

/// One channel's info inside one block header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockChannel {
    /// First 2048-byte payload entry owned by this channel in this block.
    pub start_entry: u32,
    /// Number of 2048-byte payload entries.
    pub entries: u32,
    /// Skip word; 0 in every ADPCM block and in all but 52 shipped PCM
    /// files. Read as a signed 32-bit value it is +1 where a channel's
    /// first absolute sample sits one past the block base (a 1-sample
    /// lookahead entry in an earlier block), a growing block index where
    /// that drift accumulates, and -1023 alternating with +1 in one file
    /// whose per-block counts oscillate around the nominal size. In every
    /// case the channel's own seek table stays continuous and the wave
    /// sample count stays authoritative, so the value is informational
    /// for linear playback and the decoder ignores it (verified
    /// sample-exact against vgmstream on a file with nonzero skips).
    pub skip: u32,
    /// Samples of this channel stored in this block.
    pub sample_count: u32,
}

/// One seek-table entry: absolute first and last sample of 2048 ADPCM bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeekEntry {
    /// Absolute index of the first sample covered.
    pub first: u32,
    /// Absolute index of the last sample covered.
    pub last: u32,
}

/// A parsed block header with its seek tables.
#[derive(Debug)]
pub struct Block<'a> {
    buf: &'a [u8],
    /// Block index.
    pub index: u32,
    /// Absolute byte offset of the block.
    pub off: u64,
    /// Base header size from the block (24 in files seen).
    pub base_size: u64,
    /// Channel-info end offset from the block (56 in files seen).
    pub ch_info_off: u64,
    /// Seek-table start offset from the block (56 in files seen).
    pub seek_off: u64,
    /// Per-channel infos.
    pub channels: Vec<BlockChannel>,
    /// Per-channel seek tables.
    pub seeks: Vec<Vec<SeekEntry>>,
    /// Absolute byte offset where this block's ADPCM payload starts.
    pub data_start: u64,
    /// Absolute byte offset where this block ends.
    pub block_end: u64,
}

impl<'a> Streamed<'a> {
    /// Parse a streamed container from the whole file contents.
    ///
    /// # Errors
    ///
    /// Returns an error when the input is shorter than the 48-byte header,
    /// when the block or channel counts are zero, or when the declared
    /// block layout does not end exactly at the input length.
    pub fn parse(buf: &'a [u8]) -> Result<Streamed<'a>, Error> {
        let mut cur = Cursor::new(buf);
        let table_off = cur.u64()?;
        let blocks = cur.u32()?;
        let block_chunk = cur.u32()?;
        let stream_count = cur.u32()?;
        let ch_table_off = cur.u64()?;
        let aux_off = cur.u32()?;
        let reserved_20 = cur.u32()?;
        let channels = cur.u32()?;
        let flags_28 = cur.u32()?;
        let data_off = cur.u32()?;
        if stream_count != 0 {
            return Err(Error::new(
                0x10,
                ErrorKind::Invalid,
                format!("stream count {stream_count} is non-zero (banks use the bank parser)"),
            ));
        }
        if blocks == 0 {
            return Err(Error::new(
                0x08,
                ErrorKind::Invalid,
                "zero blocks".to_string(),
            ));
        }
        if channels == 0 {
            return Err(Error::new(
                0x24,
                ErrorKind::Invalid,
                "zero channels".to_string(),
            ));
        }
        let expect_end = u64::from(data_off)
            .checked_add(
                u64::from(blocks)
                    .checked_mul(u64::from(block_chunk))
                    .ok_or_else(|| {
                        Error::new(
                            0x08,
                            ErrorKind::Invalid,
                            format!("{blocks} blocks of {block_chunk} bytes overflow"),
                        )
                    })?,
            )
            .ok_or_else(|| {
                Error::new(
                    0x2c,
                    ErrorKind::Invalid,
                    "block layout overflows".to_string(),
                )
            })?;
        let len = u64::try_from(buf.len()).unwrap_or(u64::MAX);
        if expect_end != len {
            return Err(Error::new(
                0x2c,
                ErrorKind::Invalid,
                format!(
                    "data {data_off} + {blocks} blocks of {block_chunk} ends at {expect_end}, input is {}",
                    buf.len()
                ),
            ));
        }
        if table_off > len {
            return Err(Error::new(
                0x00,
                ErrorKind::Invalid,
                format!("file table at {table_off}, input is {}", buf.len()),
            ));
        }
        Ok(Streamed {
            buf,
            table_off,
            blocks,
            block_chunk,
            ch_table_off,
            aux_off,
            reserved_20,
            channels,
            flags_28,
            data_off,
        })
    }

    /// Byte offset of the per-block file table.
    #[must_use]
    pub fn table_off(&self) -> u64 {
        self.table_off
    }

    /// Number of data blocks.
    #[must_use]
    pub fn block_count(&self) -> u32 {
        self.blocks
    }

    /// Bytes per block on disk.
    #[must_use]
    pub fn block_chunk(&self) -> u32 {
        self.block_chunk
    }

    /// Byte offset of the channel region.
    #[must_use]
    pub fn ch_table_off(&self) -> u64 {
        self.ch_table_off
    }

    /// Byte offset past the channel wave infos.
    #[must_use]
    pub fn aux_off(&self) -> u32 {
        self.aux_off
    }

    /// Reserved header word at 0x20.
    #[must_use]
    pub fn reserved_20(&self) -> u32 {
        self.reserved_20
    }

    /// Channel count.
    #[must_use]
    pub fn channel_count(&self) -> u32 {
        self.channels
    }

    /// Flags word at 0x28 (0, or 2 with an extra timestamp-looking region).
    #[must_use]
    pub fn flags_28(&self) -> u32 {
        self.flags_28
    }

    /// Byte offset of block 0.
    #[must_use]
    pub fn data_off(&self) -> u32 {
        self.data_off
    }

    /// Byte offset where the per-channel wave infos start (past the preface).
    #[must_use]
    pub fn channel_infos_off(&self) -> u64 {
        self.ch_table_off + u64::from(self.channels) * PREFACE_ENTRY_LEN
    }

    /// Read the channel-region preface: one entry per channel.
    ///
    /// # Errors
    ///
    /// Returns an error when the preface runs past the end of the input.
    pub fn preface(&self) -> Result<Vec<PrefaceChannel>, Error> {
        // The preface must fit the input; this also bounds the allocation.
        let len = u64::try_from(self.buf.len()).unwrap_or(u64::MAX);
        let room = len.saturating_sub(self.ch_table_off) / PREFACE_ENTRY_LEN;
        if u64::from(self.channels) > room {
            return Err(Error::new(
                self.ch_table_off,
                ErrorKind::Truncated,
                format!(
                    "preface of {} channels ends past input of {}",
                    self.channels,
                    self.buf.len()
                ),
            ));
        }
        let mut cur = Cursor::new(self.buf);
        cur.seek(self.ch_table_off)?;
        let mut out = Vec::with_capacity(usize::try_from(self.channels).unwrap_or(0));
        for index in 0..self.channels {
            out.push(PrefaceChannel {
                index,
                info_off: cur.u64()?,
                name_hash: cur.u32()?,
                record_len: cur.u32()?,
            });
        }
        Ok(out)
    }

    /// Per-channel record length derived from the region size: the bytes
    /// between the preface end and `aux_off` split evenly over the channels.
    /// Cross-checked against the preface entries, which must all agree.
    ///
    /// # Errors
    ///
    /// Returns an error when the region bounds disagree or a preface entry
    /// contradicts the region math.
    pub fn channel_record_len(&self) -> Result<u64, Error> {
        let start = self.channel_infos_off();
        let end = u64::from(self.aux_off);
        if end < start {
            return Err(Error::new(
                0x1c,
                ErrorKind::Invalid,
                format!("aux offset {end} before channel infos end {start}"),
            ));
        }
        let span = end - start;
        if !span.is_multiple_of(u64::from(self.channels)) {
            return Err(Error::new(
                0x1c,
                ErrorKind::Invalid,
                format!(
                    "channel span {span} not divisible by {} channels",
                    self.channels
                ),
            ));
        }
        let record_len = span / u64::from(self.channels);
        for entry in &self.preface()? {
            if u64::from(entry.record_len) != record_len {
                return Err(Error::new(
                    self.ch_table_off + u64::from(entry.index) * PREFACE_ENTRY_LEN + 12,
                    ErrorKind::Invalid,
                    format!(
                        "preface record length {} disagrees with region math {record_len}",
                        entry.record_len
                    ),
                ));
            }
            if entry.info_off != u64::from(entry.index) * record_len {
                return Err(Error::new(
                    self.ch_table_off + u64::from(entry.index) * PREFACE_ENTRY_LEN,
                    ErrorKind::Invalid,
                    format!(
                        "preface info offset {} is not channel {} times {record_len}",
                        entry.info_off, entry.index
                    ),
                ));
            }
        }
        Ok(record_len)
    }

    /// Read every channel's wave header plus its raw trailer.
    ///
    /// # Errors
    ///
    /// Returns an error when a wave header runs past the input or the
    /// record length is below the 32-byte wave header.
    pub fn channel_waves(&self) -> Result<Vec<ChannelWave<'a>>, Error> {
        let record_len = self.channel_record_len()?;
        if record_len < 32 {
            return Err(Error::new(
                self.ch_table_off,
                ErrorKind::Invalid,
                format!("channel record length {record_len} below wave header size 32"),
            ));
        }
        // `channel_record_len` ran the preface check above, so `channels`
        // fits the input and this allocation is bounded by it.
        let mut out = Vec::with_capacity(usize::try_from(self.channels).unwrap_or(0));
        for index in 0..self.channels {
            let at = self
                .channel_infos_off()
                .saturating_add(u64::from(index).saturating_mul(record_len));
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
            let trailer_len = usize::try_from(record_len - 32).unwrap_or(usize::MAX);
            let trailer = cur.bytes(trailer_len)?;
            out.push(ChannelWave {
                index,
                data_off,
                name_hash,
                size,
                sample_count,
                flags_14,
                sample_rate,
                field_1a,
                codec,
                trailer,
            });
        }
        Ok(out)
    }

    /// Raw bytes between the channel infos and the file table. Empty when the
    /// flags word is 0; timestamp-looking data otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error when the region bounds run past the input.
    pub fn aux_region(&self) -> Result<&'a [u8], Error> {
        let start = u64::from(self.aux_off);
        let len = u64::try_from(self.buf.len()).unwrap_or(u64::MAX);
        if start > self.table_off || self.table_off > len {
            return Err(Error::new(
                0x1c,
                ErrorKind::Invalid,
                format!("aux region {start}..{} out of range", self.table_off),
            ));
        }
        crate::slice_u64(self.buf, start, self.table_off).ok_or_else(|| {
            Error::new(
                start,
                ErrorKind::Truncated,
                "aux region out of range".to_string(),
            )
        })
    }

    /// Read the per-block file table.
    ///
    /// # Errors
    ///
    /// Returns an error when the table runs past the end of the input.
    pub fn file_table(&self) -> Result<Vec<BlockTableEntry>, Error> {
        // The table must fit the input; this also bounds the allocation.
        let len = u64::try_from(self.buf.len()).unwrap_or(u64::MAX);
        let room = len.saturating_sub(self.table_off) / FILE_TABLE_ENTRY_LEN;
        if u64::from(self.blocks) > room {
            return Err(Error::new(
                self.table_off,
                ErrorKind::Truncated,
                format!(
                    "file table of {} blocks ends past input of {}",
                    self.blocks,
                    self.buf.len()
                ),
            ));
        }
        let mut cur = Cursor::new(self.buf);
        cur.seek(self.table_off)?;
        let mut out = Vec::with_capacity(usize::try_from(self.blocks).unwrap_or(0));
        for _ in 0..self.blocks {
            out.push(BlockTableEntry {
                first_sample: cur.u32()?,
                sample_rate: cur.u32()?,
            });
        }
        Ok(out)
    }

    /// Parse one block header with its seek tables.
    ///
    /// # Errors
    ///
    /// Returns an error when the index is out of range or the block's
    /// headers and seek tables run past the input.
    pub fn block(&self, index: u32) -> Result<Block<'a>, Error> {
        if index >= self.blocks {
            return Err(Error::new(
                0x08,
                ErrorKind::Invalid,
                format!("block {index} of {}", self.blocks),
            ));
        }
        let off = u64::from(self.data_off)
            .saturating_add(u64::from(index).saturating_mul(u64::from(self.block_chunk)));
        let block_end = off.saturating_add(u64::from(self.block_chunk));
        let mut cur = Cursor::new(self.buf);
        cur.seek(off)?;
        let base_size = cur.u64()?;
        let ch_info_off = cur.u64()?;
        let seek_off = cur.u64()?;
        // Cap the channel vec by the input: the cursor reads below still
        // fail closed on short input.
        let ch_unit = usize::try_from(BLOCK_CH_INFO_LEN).unwrap_or(16);
        let ch_cap = usize::try_from(self.channels)
            .unwrap_or(usize::MAX)
            .min(self.buf.len() / ch_unit);
        let mut channels = Vec::with_capacity(ch_cap);
        cur.seek(off.saturating_add(base_size))?;
        for _ in 0..self.channels {
            channels.push(BlockChannel {
                start_entry: cur.u32()?,
                entries: cur.u32()?,
                skip: cur.u32()?,
                sample_count: cur.u32()?,
            });
        }
        let ch_end = off
            .saturating_add(base_size)
            .saturating_add(u64::from(self.channels).saturating_mul(BLOCK_CH_INFO_LEN));
        if off.saturating_add(seek_off) < ch_end {
            return Err(Error::new(
                off.saturating_add(16),
                ErrorKind::Invalid,
                format!(
                    "seek tables at {} overlap channel infos ending at {ch_end}",
                    off.saturating_add(seek_off)
                ),
            ));
        }
        let mut seeks = Vec::with_capacity(ch_cap);
        cur.seek(off.saturating_add(seek_off))?;
        for ch in &channels {
            // Cap the seek vec by the remaining input: the reads below still
            // fail closed on short input.
            let len = u64::try_from(self.buf.len()).unwrap_or(u64::MAX);
            let room = len.saturating_sub(cur.pos()) / 8;
            let want = u64::from(ch.entries).min(room);
            let mut table = Vec::with_capacity(usize::try_from(want).unwrap_or(0));
            for _ in 0..ch.entries {
                table.push(SeekEntry {
                    first: cur.u32()?,
                    last: cur.u32()?,
                });
            }
            seeks.push(table);
        }
        let data_start = align_up(cur.pos(), SEEK_ENTRY_BYTES);
        Ok(Block {
            buf: self.buf,
            index,
            off,
            base_size,
            ch_info_off,
            seek_off,
            channels,
            seeks,
            data_start,
            block_end,
        })
    }

    /// Decode one channel of the whole file to 16-bit PCM.
    ///
    /// The codec comes from the channel's wave header: ADPCM decodes with
    /// state carried across block boundaries (the game feeds the decoder
    /// linearly; only a mid-stream seek starts a block from a fresh state),
    /// while PCM channels concatenate. See [`Block::decode_channel`] for
    /// seek-style single-block decode.
    ///
    /// The wave header's sample count is the authoritative length: block
    /// sample counts occasionally sum a few samples short of it (seen in a
    /// PCM file whose last block disagrees across channels), so the output
    /// is padded with zeros or truncated to exactly that many samples.
    ///
    /// # Errors
    ///
    /// Returns an error when the channel is out of range, a block fails
    /// to parse, the codec has no decoder, or the declared sample count
    /// exceeds the policy cap.
    pub fn decode_channel(&self, channel: u32) -> Result<Vec<i16>, Error> {
        if channel >= self.channels {
            return Err(Error::new(
                0x24,
                ErrorKind::Invalid,
                format!("channel {channel} of {}", self.channels),
            ));
        }
        let waves = self.channel_waves()?;
        let wave = &waves[channel as usize];
        if wave.sample_count > MAX_DECODED_SAMPLES {
            return Err(Error::new(
                0x24,
                ErrorKind::Invalid,
                format!(
                    "channel sample count {} exceeds policy cap {MAX_DECODED_SAMPLES}",
                    wave.sample_count
                ),
            ));
        }
        let mut out = Vec::new();
        let mut state = ImaState::new();
        for b in 0..self.blocks {
            let block = self.block(b)?;
            let channel_usize = usize::try_from(channel).unwrap_or(usize::MAX);
            block.decode_channel_into(channel_usize, wave.codec, &mut state, &mut out)?;
        }
        out.resize(usize::try_from(wave.sample_count).unwrap_or(0), 0);
        Ok(out)
    }

    /// Decode every channel to 16-bit PCM.
    ///
    /// # Errors
    ///
    /// Returns the first channel error; see [`Streamed::decode_channel`].
    pub fn decode_all(&self) -> Result<Vec<Vec<i16>>, Error> {
        (0..self.channels).map(|c| self.decode_channel(c)).collect()
    }
}

impl<'a> Block<'a> {
    /// Borrow one channel's payload bytes (ADPCM or PCM) in this block.
    ///
    /// # Errors
    ///
    /// Returns an error when the channel is out of range or its payload
    /// runs past the end of the block.
    pub fn channel_data(&self, channel: usize) -> Result<&'a [u8], Error> {
        let ch = self.channels.get(channel).ok_or_else(|| {
            Error::new(
                self.off,
                ErrorKind::Invalid,
                format!("channel {channel} of {}", self.channels.len()),
            )
        })?;
        let start = self
            .data_start
            .saturating_add(u64::from(ch.start_entry).saturating_mul(SEEK_ENTRY_BYTES));
        let end = start.saturating_add(u64::from(ch.entries).saturating_mul(SEEK_ENTRY_BYTES));
        if end > self.block_end {
            return Err(Error::new(
                start,
                ErrorKind::Truncated,
                format!(
                    "block {} channel {channel} payload ends at {end}, block ends at {}",
                    self.index, self.block_end
                ),
            ));
        }
        crate::slice_u64(self.buf, start, end).ok_or_else(|| {
            Error::new(
                start,
                ErrorKind::Truncated,
                format!(
                    "block {} channel {channel} payload out of range",
                    self.index
                ),
            )
        })
    }

    /// Decode one channel of this block, appending to `out`.
    ///
    /// `codec` is the channel's codec tag from its wave header. ADPCM
    /// advances `state` so the caller can chain blocks with continuous
    /// state; PCM ignores the state and appends copied samples.
    ///
    /// A block sample count that runs past the payload's capacity (seen in
    /// one shipped PCM file) is clamped to what is there rather than
    /// failing; [`Streamed::decode_channel`] pads the channel to its
    /// declared length afterwards.
    ///
    /// # Errors
    ///
    /// Returns an error when the channel is out of range, its payload
    /// runs past the block, or the codec has no decoder.
    pub fn decode_channel_into(
        &self,
        channel: usize,
        codec: u32,
        state: &mut ImaState,
        out: &mut Vec<i16>,
    ) -> Result<(), Error> {
        let info = self.channels.get(channel).ok_or_else(|| {
            Error::new(
                self.off,
                ErrorKind::Invalid,
                format!("channel {channel} of {}", self.channels.len()),
            )
        })?;
        let data = self.channel_data(channel)?;
        let capacity = match codec {
            CODEC_ADPCM => data.len().saturating_mul(2),
            CODEC_PCM16 => data.len() / 2,
            codec => {
                return Err(Error::new(
                    self.off,
                    ErrorKind::Unsupported,
                    format!("codec {codec:#x} has no decoder"),
                ));
            }
        };
        let want = usize::try_from(info.sample_count)
            .unwrap_or(usize::MAX)
            .min(capacity);
        if codec == CODEC_ADPCM {
            state.decode_bytes(data, want, out)
        } else {
            out.extend(decode_pcm16(data, want, self.off)?);
            Ok(())
        }
    }

    /// Decode one channel of this block from a fresh ADPCM state, as a
    /// mid-stream seek would (the first ~100 samples are convergence
    /// lead-in, not exact audio). `codec` is the channel's codec tag.
    ///
    /// # Errors
    ///
    /// Returns an error when the channel is out of range, its payload
    /// runs past the block, or the codec has no decoder.
    pub fn decode_channel(&self, channel: usize, codec: u32) -> Result<Vec<i16>, Error> {
        let mut out = Vec::new();
        self.decode_channel_into(channel, codec, &mut ImaState::new(), &mut out)?;
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal streamed file: 1 channel, 1 block, record length 48
    /// (32-byte wave header + 16-byte trailer), one seek entry covering 2048
    /// bytes / 4096 samples.
    fn tiny_streamed() -> Vec<u8> {
        // header 48 + preface 16 + ch info 48 + table 8; data at 2048.
        let data_off = 2048u32;
        let mut b = vec![0u8; 2048 + 4096];
        b[0..8].copy_from_slice(&112u64.to_le_bytes()); // table_off
        b[8..12].copy_from_slice(&1u32.to_le_bytes()); // blocks
        b[12..16].copy_from_slice(&4096u32.to_le_bytes()); // chunk
        b[16..20].copy_from_slice(&0u32.to_le_bytes()); // stream_count 0
        b[20..28].copy_from_slice(&48u64.to_le_bytes()); // ch_table_off
        b[28..32].copy_from_slice(&112u32.to_le_bytes()); // aux_off
        b[36..40].copy_from_slice(&1u32.to_le_bytes()); // channels
        b[44..48].copy_from_slice(&data_off.to_le_bytes());
        // preface at 48: one entry (info_off 0, hash, record_len 48)
        b[48..56].copy_from_slice(&0u64.to_le_bytes());
        b[56..60].copy_from_slice(&0x1111u32.to_le_bytes());
        b[60..64].copy_from_slice(&48u32.to_le_bytes());
        // channel wave info at 64: size=2048, samples=4096, rate=32000, codec
        b[64 + 12..64 + 16].copy_from_slice(&2048u32.to_le_bytes());
        b[64 + 16..64 + 20].copy_from_slice(&4096u32.to_le_bytes());
        b[64 + 24..64 + 26].copy_from_slice(&32000u16.to_le_bytes());
        b[64 + 28..64 + 32].copy_from_slice(&CODEC_ADPCM.to_le_bytes());
        // file table at 112: first_sample 0, rate 32000
        b[112..116].copy_from_slice(&0u32.to_le_bytes());
        b[116..120].copy_from_slice(&32000u32.to_le_bytes());
        // block at 2048: base [24, 56, 56], ch [0, 1, 0, 4096], seek [0, 4095]
        b[2048..2056].copy_from_slice(&24u64.to_le_bytes());
        b[2056..2064].copy_from_slice(&56u64.to_le_bytes());
        b[2064..2072].copy_from_slice(&56u64.to_le_bytes());
        b[2072..2076].copy_from_slice(&0u32.to_le_bytes());
        b[2076..2080].copy_from_slice(&1u32.to_le_bytes());
        b[2080..2084].copy_from_slice(&0u32.to_le_bytes());
        b[2084..2088].copy_from_slice(&4096u32.to_le_bytes());
        b[2104..2108].copy_from_slice(&0u32.to_le_bytes());
        b[2108..2112].copy_from_slice(&4095u32.to_le_bytes());
        // payload at 4096: first bytes 0x77 0x77, rest silence
        b[4096] = 0x77;
        b[4097] = 0x77;
        b
    }

    #[test]
    fn parses_hand_built_stream() {
        let b = tiny_streamed();
        let s = Streamed::parse(&b).unwrap();
        assert_eq!(s.block_count(), 1);
        assert_eq!(s.channel_count(), 1);
        assert_eq!(s.channel_record_len().unwrap(), 48);
        let preface = s.preface().unwrap();
        assert_eq!(preface.len(), 1);
        assert_eq!(preface[0].name_hash, 0x1111);
        let waves = s.channel_waves().unwrap();
        assert_eq!(waves.len(), 1);
        assert_eq!(waves[0].sample_count, 4096);
        assert_eq!(waves[0].sample_rate, 32000);
        assert_eq!(waves[0].trailer.len(), 16);
        assert!(s.aux_region().unwrap().is_empty());
        let table = s.file_table().unwrap();
        assert_eq!(
            table,
            vec![BlockTableEntry {
                first_sample: 0,
                sample_rate: 32000
            }]
        );
        let block = s.block(0).unwrap();
        assert_eq!(block.data_start, 4096);
        assert_eq!(
            block.channels[0],
            BlockChannel {
                start_entry: 0,
                entries: 1,
                skip: 0,
                sample_count: 4096
            }
        );
        assert_eq!(
            block.seeks[0],
            vec![SeekEntry {
                first: 0,
                last: 4095
            }]
        );
        let pcm = s.decode_channel(0).unwrap();
        assert_eq!(pcm.len(), 4096);
        assert_eq!(pcm[0], 11);
        assert_eq!(pcm[3], 240);
    }

    #[test]
    fn overlong_block_count_clamps_to_payload() {
        let mut b = tiny_streamed();
        b[2084..2088].copy_from_slice(&5000u32.to_le_bytes()); // beyond 4096 capacity
        let s = Streamed::parse(&b).unwrap();
        let block = s.block(0).unwrap();
        let pcm = block.decode_channel(0, CODEC_ADPCM).unwrap();
        assert_eq!(pcm.len(), 4096);
    }

    #[test]
    fn rejects_bad_block_math() {
        let mut b = tiny_streamed();
        b[12..16].copy_from_slice(&2048u32.to_le_bytes()); // chunk no longer fits
        assert_eq!(Streamed::parse(&b).unwrap_err().kind(), ErrorKind::Invalid);
    }

    #[test]
    fn block_pcm_decode_copies_pairs() {
        let mut b = tiny_streamed();
        // Rework the block channel as 1024 PCM samples = 2048 payload bytes.
        b[2084..2088].copy_from_slice(&1024u32.to_le_bytes());
        let s = Streamed::parse(&b).unwrap();
        let block = s.block(0).unwrap();
        let pcm = block.decode_channel(0, CODEC_PCM16).unwrap();
        assert_eq!(pcm.len(), 1024);
        assert_eq!(pcm[0], 0x7777);
        assert_eq!(pcm[1], 0x0000);
        assert_eq!(
            block.decode_channel(0, 0x200).unwrap_err().kind(),
            ErrorKind::Unsupported
        );
    }
}
