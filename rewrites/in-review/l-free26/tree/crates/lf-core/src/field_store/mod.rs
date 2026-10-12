//! Field stores: one parameter written into a field of an object.
//!
//! Many small routines of the original share one shape. Each takes an object
//! and one parameter, writes the parameter into one field at a fixed offset
//! of the object, and returns one of four things: zero, the parameter, the
//! parameter's low byte, or nothing. The field is one byte or one 32-bit
//! word. Nothing else of the object is read or written.
//!
//! [`ObjectBytes`] owns the bytes of one object. [`FieldStore`] describes one
//! such routine as data: the offset of its field, the width of the field, and
//! what the routine returns. [`ObjectBytes::store`] is the routine itself, one
//! method for every instance. The instances the verified rewrites were
//! checked against are listed in [`instances`].
//!
//! Specification of [`ObjectBytes::store`]:
//!
//! - A byte field receives the low eight bits of the parameter.
//! - A word field receives the parameter as four bytes, least significant
//!   byte first, which is how the 32-bit target lays out a word.
//! - The return value is `Some(0)` for [`StoreResult::Zero`], `Some(value)`
//!   for [`StoreResult::Value`], `Some(value & 0xFF)` for
//!   [`StoreResult::LowByte`] and `None` for [`StoreResult::Nothing`].
//! - A field that lies outside the object is a caller error and panics: the
//!   original would have written outside the object, and the lift fails
//!   loudly instead of guessing.
//! - Nothing here loops or allocates per call.
//!
//! Floating-point parameters are passed as their 32 bits; a store copies the
//! bits, so no value is changed by the store.

pub mod instances;

/// Bytes in one 32-bit word.
const WORD_BYTES: usize = 4;

/// The width of the field a store writes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldWidth {
    /// One byte: the low eight bits of the parameter.
    Byte,
    /// One 32-bit word, least significant byte first.
    Word,
}

/// What a store hands back to its caller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoreResult {
    /// Zero, whatever the parameter was.
    Zero,
    /// The parameter, unchanged.
    Value,
    /// The parameter's low byte, as a word.
    LowByte,
    /// Nothing: the routine returns no value.
    Nothing,
}

/// One field store, as data: where it writes, how wide, and what it returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldStore {
    /// Byte offset of the field from the start of the object.
    pub offset: usize,
    /// Width of the field.
    pub width: FieldWidth,
    /// What the store returns.
    pub result: StoreResult,
}

/// The bytes of one object. Field offsets count from byte zero of the object.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectBytes {
    bytes: Vec<u8>,
}

impl ObjectBytes {
    /// An object of `len` bytes, all zero.
    #[must_use]
    pub fn zeroed(len: usize) -> Self {
        Self {
            bytes: vec![0; len],
        }
    }

    /// An object holding a copy of `bytes`.
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec(),
        }
    }

    /// The object's bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The object's length in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// True when the object has no bytes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// The byte at `offset`.
    ///
    /// # Panics
    ///
    /// When `offset` is outside the object.
    #[must_use]
    pub fn byte(&self, offset: usize) -> u8 {
        self.bytes[offset]
    }

    /// The 32-bit word at `offset`, least significant byte first.
    ///
    /// # Panics
    ///
    /// When the word does not lie inside the object.
    #[must_use]
    pub fn word(&self, offset: usize) -> u32 {
        let end = field_end(offset, WORD_BYTES, self.bytes.len());
        let mut word = [0u8; WORD_BYTES];
        word.copy_from_slice(&self.bytes[offset..end]);
        u32::from_le_bytes(word)
    }

    /// Writes `bytes` into the object at `offset`, one byte after another. Only those bytes change.
    ///
    /// # Panics
    ///
    /// When the bytes do not lie inside the object.
    pub fn write(&mut self, offset: usize, bytes: &[u8]) {
        let end = field_end(offset, bytes.len(), self.bytes.len());
        self.bytes[offset..end].copy_from_slice(bytes);
    }

    /// Writes `value` into the field `store` describes and returns what the
    /// store hands back. Only the field is written.
    ///
    /// # Panics
    ///
    /// When the field does not lie inside the object.
    pub fn store(&mut self, store: FieldStore, value: u32) -> Option<u32> {
        match store.width {
            FieldWidth::Byte => {
                let end = field_end(store.offset, 1, self.bytes.len());
                self.bytes[store.offset..end].copy_from_slice(&[value as u8]);
            }
            FieldWidth::Word => {
                let end = field_end(store.offset, WORD_BYTES, self.bytes.len());
                self.bytes[store.offset..end].copy_from_slice(&value.to_le_bytes());
            }
        }
        match store.result {
            StoreResult::Zero => Some(0),
            StoreResult::Value => Some(value),
            StoreResult::LowByte => Some(value & 0xFF),
            StoreResult::Nothing => None,
        }
    }
}

/// The end of a field of `width` bytes at `offset`, checked against the
/// object's length. The sum is compared as a length, so it cannot wrap.
fn field_end(offset: usize, width: usize, len: usize) -> usize {
    assert!(
        offset <= len && width <= len - offset,
        "field of {width} bytes at offset {offset} lies outside an object of {len} bytes"
    );
    offset + width
}

#[cfg(test)]
mod tests {
    use super::{FieldStore, FieldWidth, ObjectBytes, StoreResult};

    const BYTE_ZERO: FieldStore = FieldStore {
        offset: 2,
        width: FieldWidth::Byte,
        result: StoreResult::Zero,
    };
    const WORD_VALUE: FieldStore = FieldStore {
        offset: 4,
        width: FieldWidth::Word,
        result: StoreResult::Value,
    };

    #[test]
    fn byte_store_writes_only_the_low_byte() {
        let mut object = ObjectBytes::zeroed(8);
        let before = object.clone();
        assert_eq!(object.store(BYTE_ZERO, 0x1234_56AB), Some(0));
        assert_eq!(object.byte(2), 0xAB);
        assert_eq!(object.byte(1), before.byte(1));
        assert_eq!(object.byte(3), before.byte(3));
    }

    #[test]
    fn word_store_writes_four_bytes_little_endian() {
        let mut object = ObjectBytes::zeroed(8);
        assert_eq!(object.store(WORD_VALUE, 0x0403_0201), Some(0x0403_0201));
        assert_eq!(object.bytes(), &[0, 0, 0, 0, 1, 2, 3, 4]);
        assert_eq!(object.word(4), 0x0403_0201);
    }

    #[test]
    fn low_byte_result_masks_the_parameter() {
        let store = FieldStore {
            offset: 0,
            width: FieldWidth::Byte,
            result: StoreResult::LowByte,
        };
        let mut object = ObjectBytes::zeroed(1);
        assert_eq!(object.store(store, 0xFFFF_FF7E), Some(0x7E));
    }

    #[test]
    fn nothing_result_returns_none() {
        let store = FieldStore {
            offset: 0,
            width: FieldWidth::Word,
            result: StoreResult::Nothing,
        };
        let mut object = ObjectBytes::zeroed(4);
        assert_eq!(object.store(store, 9), None);
        assert_eq!(object.word(0), 9);
    }

    #[test]
    fn field_at_the_very_end_of_the_object_is_allowed() {
        let store = FieldStore {
            offset: 4,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        };
        let mut object = ObjectBytes::zeroed(8);
        assert_eq!(object.store(store, u32::MAX), Some(0));
        assert_eq!(object.word(4), u32::MAX);
    }

    #[test]
    #[should_panic(expected = "lies outside an object")]
    fn word_field_past_the_end_panics() {
        let store = FieldStore {
            offset: 5,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        };
        let mut object = ObjectBytes::zeroed(8);
        let _ = object.store(store, 1);
    }

    #[test]
    #[should_panic(expected = "lies outside an object")]
    fn huge_offset_panics_without_wrapping() {
        let store = FieldStore {
            offset: usize::MAX,
            width: FieldWidth::Byte,
            result: StoreResult::Zero,
        };
        let mut object = ObjectBytes::zeroed(8);
        let _ = object.store(store, 1);
    }

    #[test]
    fn every_listed_instance_fits_its_own_field_table() {
        for named in super::instances::STORES.iter() {
            let width = match named.store.width {
                FieldWidth::Byte => 1,
                FieldWidth::Word => 4,
            };
            assert!(named.store.offset + width <= 0x2000, "{}", named.name);
        }
    }
}
