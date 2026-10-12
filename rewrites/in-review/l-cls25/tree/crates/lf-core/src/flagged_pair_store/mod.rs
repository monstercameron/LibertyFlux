//! Flagged pair stores: two parameters written into two fields of one object,
//! and one flag byte of the object set or updated on the way.
//!
//! Several small routines of the original take an object and two 32-bit
//! parameters. They write the first parameter into one field, set a flag byte
//! of the object, and write the second parameter into another field. Each
//! returns a fixed value whatever the parameters were. The routines differ in
//! the two field offsets, in the flag byte's offset, in whether the flag is set
//! to a value or has a mask ORed into it, and in the value returned.
//! [`FlaggedPairStore`] holds one such routine as data, and
//! [`FlaggedPairStore::apply`] is the routine. The existing
//! [`crate::field_pair_store`] module covers the same pair of stores without
//! the flag.
//!
//! The object is seen as its bytes, little endian. No address appears here.
//!
//! Specification of [`FlaggedPairStore::apply`]:
//!
//! - The first parameter is written, as four bytes least significant first, at
//!   the first field's offset.
//! - The flag byte is then updated, as [`FlagAction`] says: [`FlagAction::Set`]
//!   writes its value to the byte; [`FlagAction::Or`] reads the byte and writes
//!   it back with its mask ORed in.
//! - The second parameter is written, as four bytes least significant first, at
//!   the second field's offset.
//! - The first and second fields and the flag byte are distinct bytes, so the
//!   order of the three updates is not observable. Nothing else is written.
//! - The result is the value the routine returns, [`FlaggedPairStore::returns`],
//!   whatever the parameters were.
//! - A field or flag outside the object is a caller error and panics.

use crate::field_store::ObjectBytes;

/// How the flag byte is updated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlagAction {
    /// The byte at `offset` is written with `value`.
    Set {
        /// Byte offset of the flag.
        offset: usize,
        /// The value written.
        value: u8,
    },
    /// The byte at `offset` is read and written back with `mask` ORed in.
    Or {
        /// Byte offset of the flag.
        offset: usize,
        /// The bits ORed in.
        mask: u8,
    },
}

/// One flagged pair store, as data: the field offsets, the flag update and the
/// value returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlaggedPairStore {
    /// Byte offset of the field that receives the first parameter.
    pub first_offset: usize,
    /// Byte offset of the field that receives the second parameter.
    pub second_offset: usize,
    /// How the flag byte is updated.
    pub flag: FlagAction,
    /// The value the routine returns, whatever the parameters were.
    pub returns: u32,
}

impl FlaggedPairStore {
    /// Applies the routine to `object` with the two parameters and returns the
    /// value this routine hands back.
    ///
    /// # Panics
    ///
    /// When a field or the flag lies outside the object.
    pub fn apply(&self, object: &mut ObjectBytes, first: u32, second: u32) -> u32 {
        object.write(self.first_offset, &first.to_le_bytes());
        match self.flag {
            FlagAction::Set { offset, value } => object.write(offset, &[value]),
            FlagAction::Or { offset, mask } => {
                let current = object.byte(offset);
                object.write(offset, &[current | mask]);
            }
        }
        object.write(self.second_offset, &second.to_le_bytes());
        self.returns
    }
}

#[cfg(test)]
mod tests {
    use super::{FlagAction, FlaggedPairStore};
    use crate::field_store::ObjectBytes;

    #[test]
    fn set_flag_writes_the_value_between_the_two_words() {
        let store = FlaggedPairStore {
            first_offset: 0,
            second_offset: 8,
            flag: FlagAction::Set {
                offset: 4,
                value: 1,
            },
            returns: 0,
        };
        let mut object = ObjectBytes::zeroed(12);
        assert_eq!(store.apply(&mut object, 0x0403_0201, 0x0807_0605), 0);
        assert_eq!(object.bytes(), &[1, 2, 3, 4, 1, 0, 0, 0, 5, 6, 7, 8]);
    }

    #[test]
    fn or_flag_keeps_the_bits_it_does_not_mask() {
        let store = FlaggedPairStore {
            first_offset: 0,
            second_offset: 4,
            flag: FlagAction::Or {
                offset: 8,
                mask: 0x08,
            },
            returns: 1,
        };
        let mut bytes = vec![0u8; 12];
        bytes[8] = 0xF1;
        let mut object = ObjectBytes::from_bytes(&bytes);
        assert_eq!(store.apply(&mut object, 7, 9), 1);
        assert_eq!(object.byte(8), 0xF9);
    }
}
