//! Speech lookup files (`speech.dat` and per-episode variants).
//!
//! Unlike the versioned files these have no suffix header: a variation-data
//! blob, a flat context table, a voice table and a bank-name heap. Voices
//! and contexts are anonymous hashes; only the bank names are stored.

use crate::{Cursor, Error, ErrorKind};

/// Size of one context record in bytes.
pub const CONTEXT_SIZE: usize = 14;

/// Size of one voice record in bytes.
pub const VOICE_SIZE: usize = 10;

/// A parsed speech file borrowing its input.
#[derive(Debug)]
pub struct SpeechFile<'a> {
    variation_data: &'a [u8],
    contexts: Vec<SpeechContext>,
    voices: Vec<Voice>,
    banks: Vec<String>,
}

/// One context: a voice line in a bank with per-take variation bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeechContext {
    bank_index: u32,
    variation_offset: i32,
    name_hash: u32,
    context_data: u8,
    variation_count: u8,
}

impl SpeechContext {
    /// Index into [`SpeechFile::banks`].
    #[must_use]
    pub fn bank_index(&self) -> u32 {
        self.bank_index
    }

    /// Byte offset into the variation blob, or -1 when the context has no
    /// variation bytes.
    #[must_use]
    pub fn variation_offset(&self) -> i32 {
        self.variation_offset
    }

    /// Anonymous name hash of the context.
    #[must_use]
    pub fn name_hash(&self) -> u32 {
        self.name_hash
    }

    /// Meaning unknown; small values in shipped files.
    #[must_use]
    pub fn context_data(&self) -> u8 {
        self.context_data
    }

    /// How many variation bytes this context owns.
    #[must_use]
    pub fn variation_count(&self) -> u8 {
        self.variation_count
    }
}

/// One voice: a run of contexts laid out back to back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Voice {
    name_hash: u32,
    context_count: u16,
    first_context: u32,
}

impl Voice {
    /// Anonymous name hash of the voice.
    #[must_use]
    pub fn name_hash(&self) -> u32 {
        self.name_hash
    }

    /// How many contexts the voice owns.
    #[must_use]
    pub fn context_count(&self) -> u16 {
        self.context_count
    }

    /// Index of the voice's first context in [`SpeechFile::contexts`].
    #[must_use]
    pub fn first_context(&self) -> u32 {
        self.first_context
    }
}

impl<'a> SpeechFile<'a> {
    /// Parse a whole file. All cross-references (voice runs, bank indices,
    /// variation ranges) are checked; a dangling one is
    /// [`ErrorKind::Invalid`].
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(buf: &'a [u8]) -> Result<SpeechFile<'a>, Error> {
        let mut cur = Cursor::new(buf);
        let var_len = cur.u32("variation size")? as usize;
        let variation_data = cur.bytes(var_len, "variation data")?;

        let context_count = cur.u32("context count")? as usize;
        let mut contexts =
            Vec::with_capacity(context_count.min(cur.remaining() / CONTEXT_SIZE + 1));
        for _ in 0..context_count {
            contexts.push(SpeechContext {
                bank_index: cur.u32("bank index")?,
                variation_offset: cur.i32("variation offset")?,
                name_hash: cur.u32("context hash")?,
                context_data: cur.u8("context data")?,
                variation_count: cur.u8("variation count")?,
            });
        }

        let voice_count = cur.u32("voice count")? as usize;
        let mut voices = Vec::with_capacity(voice_count.min(cur.remaining() / VOICE_SIZE + 1));
        for _ in 0..voice_count {
            let at = cur.pos();
            let first_byte_offset = cur.u32("first context offset")?;
            let name_hash = cur.u32("voice hash")?;
            let context_count = cur.u16("voice context count")?;
            if !(first_byte_offset as usize).is_multiple_of(CONTEXT_SIZE) {
                return Err(Error::new(
                    at as u64,
                    ErrorKind::Invalid,
                    "voice context offset not a multiple of 14",
                ));
            }
            voices.push(Voice {
                name_hash,
                context_count,
                first_context: u32::try_from(first_byte_offset as usize / CONTEXT_SIZE)
                    .unwrap_or(u32::MAX),
            });
        }

        let bank_count = cur.u32("bank count")? as usize;
        let mut bank_offsets = Vec::with_capacity(bank_count.min(cur.remaining() / 4 + 1));
        for _ in 0..bank_count {
            bank_offsets.push(cur.u32("bank offset")? as usize);
        }
        let heap_start = cur.pos();
        let heap = cur.bytes(cur.remaining(), "bank heap")?;
        let mut banks = Vec::with_capacity(bank_offsets.len());
        for off in &bank_offsets {
            banks.push(heap_string(heap, *off, heap_start)?);
        }

        let file = SpeechFile {
            variation_data,
            contexts,
            voices,
            banks,
        };
        file.check_links()?;
        Ok(file)
    }

    fn check_links(&self) -> Result<(), Error> {
        for (i, voice) in self.voices.iter().enumerate() {
            let end = voice.first_context as usize + voice.context_count as usize;
            if end > self.contexts.len() {
                return Err(Error::new(
                    0,
                    ErrorKind::Invalid,
                    format!("voice {i} context run past end of context table"),
                ));
            }
        }
        for (i, ctx) in self.contexts.iter().enumerate() {
            if ctx.bank_index as usize >= self.banks.len() {
                return Err(Error::new(
                    0,
                    ErrorKind::Invalid,
                    format!("context {i} bank index past end of bank table"),
                ));
            }
            if ctx.variation_offset >= 0 {
                let start = usize::try_from(ctx.variation_offset).map_err(|_| {
                    Error::new(
                        0,
                        ErrorKind::Invalid,
                        format!("context {i} negative variation offset"),
                    )
                })?;
                let end = start + ctx.variation_count as usize;
                if end > self.variation_data.len() {
                    return Err(Error::new(
                        0,
                        ErrorKind::Invalid,
                        format!("context {i} variation run past end of variation blob"),
                    ));
                }
            }
        }
        Ok(())
    }

    /// Raw variation-data blob.
    #[must_use]
    pub fn variation_data(&self) -> &'a [u8] {
        self.variation_data
    }

    /// All contexts in file order (laid out voice by voice).
    #[must_use]
    pub fn contexts(&self) -> &[SpeechContext] {
        &self.contexts
    }

    /// All voices in file order.
    #[must_use]
    pub fn voices(&self) -> &[Voice] {
        &self.voices
    }

    /// Bank-name heap (`ARCHIVE\BANK` paths) in table order.
    #[must_use]
    pub fn banks(&self) -> &[String] {
        &self.banks
    }

    /// The contexts owned by `voice`.
    #[must_use]
    pub fn contexts_of(&self, voice: &Voice) -> &[SpeechContext] {
        let start = voice.first_context as usize;
        &self.contexts[start..start + voice.context_count as usize]
    }

    /// The bank name a context points at.
    #[must_use]
    pub fn bank_of(&self, ctx: &SpeechContext) -> &str {
        &self.banks[ctx.bank_index as usize]
    }

    /// The variation bytes a context owns (empty when the offset is -1).
    #[must_use]
    pub fn variations_of(&self, ctx: &SpeechContext) -> &'a [u8] {
        if ctx.variation_offset < 0 {
            return &[];
        }
        let start = usize::try_from(ctx.variation_offset).unwrap_or(0);
        &self.variation_data[start..start + ctx.variation_count as usize]
    }
}

fn heap_string(heap: &[u8], off: usize, base: usize) -> Result<String, Error> {
    if off >= heap.len() {
        return Err(Error::new(
            (base + off) as u64,
            ErrorKind::Invalid,
            "bank offset outside heap",
        ));
    }
    let end = heap[off..].iter().position(|b| *b == 0).ok_or_else(|| {
        Error::new(
            (base + off) as u64,
            ErrorKind::Invalid,
            "unterminated bank name",
        )
    })?;
    String::from_utf8(heap[off..off + end].to_vec()).map_err(|_| {
        Error::new(
            (base + off) as u64,
            ErrorKind::Invalid,
            "bank name not UTF-8",
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Hand-built speech file: 3 variation bytes, 2 contexts, 1 voice
    /// owning both, 2 banks.
    fn tiny_speech() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&3u32.to_le_bytes());
        b.extend_from_slice(&[9, 8, 7]);
        b.extend_from_slice(&2u32.to_le_bytes());
        // ctx0: bank 1, var off 0, count 2, hash, data
        b.extend_from_slice(&1u32.to_le_bytes());
        b.extend_from_slice(&0i32.to_le_bytes());
        b.extend_from_slice(&0x1111u32.to_le_bytes());
        b.extend_from_slice(&[4, 2]);
        // ctx1: bank 0, no variations
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&(-1i32).to_le_bytes());
        b.extend_from_slice(&0x2222u32.to_le_bytes());
        b.extend_from_slice(&[5, 0]);
        // voice: first ctx byte offset 0, hash, 2 contexts
        b.extend_from_slice(&1u32.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&0x3333u32.to_le_bytes());
        b.extend_from_slice(&2u16.to_le_bytes());
        // banks
        b.extend_from_slice(&2u32.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&4u32.to_le_bytes());
        b.extend_from_slice(b"R\\A\0S\\B\0");
        b
    }

    #[test]
    fn parses_tiny_speech() {
        let b = tiny_speech();
        let s = SpeechFile::parse(&b).unwrap();
        assert_eq!(s.variation_data(), &[9, 8, 7]);
        assert_eq!(s.contexts().len(), 2);
        assert_eq!(s.voices().len(), 1);
        assert_eq!(s.banks(), &["R\\A".to_string(), "S\\B".to_string()]);
        let v = &s.voices()[0];
        assert_eq!(v.name_hash(), 0x3333);
        assert_eq!(s.contexts_of(v).len(), 2);
        assert_eq!(s.bank_of(&s.contexts()[0]), "S\\B");
        assert_eq!(s.variations_of(&s.contexts()[0]), &[9, 8]);
        assert!(s.variations_of(&s.contexts()[1]).is_empty());
    }

    #[test]
    fn rejects_bad_links() {
        let b = tiny_speech();
        // voice context count 3 overruns the 2-entry table
        let mut bad = b.clone();
        let at = 4 + 3 + 4 + 28 + 4 + 8;
        bad[at..at + 2].copy_from_slice(&3u16.to_le_bytes());
        assert_eq!(
            SpeechFile::parse(&bad).unwrap_err().kind(),
            ErrorKind::Invalid
        );
        assert_eq!(
            SpeechFile::parse(&b[..b.len() - 2]).unwrap_err().kind(),
            ErrorKind::Invalid
        );
    }
}
