//! Field-pair stores: two parameters written into two fields of one object, one returned or zero returned.
//!
//! Several small routines of the original take an object and two 32-bit parameters, write the first into one
//! field and the second into another, and return either the second parameter or zero. The two field offsets
//! and the return are the only things that differ between the routines. [`FieldPairStore`] holds one such
//! routine as data, and [`FieldPairStore::apply`] is the routine.
//!
//! Specification of [`FieldPairStore::apply`]:
//!
//! - The first parameter is written, as four bytes least significant first, at the first field's offset.
//! - The second parameter is written at the second field's offset. The two fields do not overlap.
//! - Nothing else is written. The result is the second parameter for [`PairResult::Second`] and zero for
//!   [`PairResult::Zero`].
//! - A field outside the object is a caller error and panics.

use crate::field_store::ObjectBytes;

/// What a pair store hands back to its caller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairResult {
    /// The second parameter, unchanged.
    Second,
    /// Zero, whatever the parameters were.
    Zero,
}

/// One pair store, as data: the offsets of its two fields and what it returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldPairStore {
    /// Byte offset of the field that receives the first parameter.
    pub first_offset: usize,
    /// Byte offset of the field that receives the second parameter.
    pub second_offset: usize,
    /// What the store returns.
    pub result: PairResult,
}

impl FieldPairStore {
    /// Stores `first` and `second` into the object and returns the result this routine hands back.
    ///
    /// # Panics
    ///
    /// When either field lies outside the object.
    pub fn apply(&self, object: &mut ObjectBytes, first: u32, second: u32) -> u32 {
        object.write(self.first_offset, &first.to_le_bytes());
        object.write(self.second_offset, &second.to_le_bytes());
        match self.result {
            PairResult::Second => second,
            PairResult::Zero => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FieldPairStore, PairResult};
    use crate::field_store::ObjectBytes;

    #[test]
    fn both_fields_receive_their_parameter_and_the_second_is_returned() {
        let store = FieldPairStore {
            first_offset: 0,
            second_offset: 4,
            result: PairResult::Second,
        };
        let mut object = ObjectBytes::zeroed(8);
        let result = store.apply(&mut object, 0x0102_0304, 0x0A0B_0C0D);
        assert_eq!(result, 0x0A0B_0C0D);
        assert_eq!(object.bytes(), &[4, 3, 2, 1, 0x0D, 0x0C, 0x0B, 0x0A]);
    }

    #[test]
    fn a_zero_result_ignores_the_parameters_in_the_return() {
        let store = FieldPairStore {
            first_offset: 2,
            second_offset: 8,
            result: PairResult::Zero,
        };
        let mut object = ObjectBytes::zeroed(12);
        assert_eq!(store.apply(&mut object, 7, 9), 0);
        assert_eq!(object.word(8), 9);
    }

    #[test]
    #[should_panic]
    fn a_field_past_the_end_panics() {
        let store = FieldPairStore {
            first_offset: 0,
            second_offset: 6,
            result: PairResult::Zero,
        };
        let mut object = ObjectBytes::zeroed(8);
        let _ = store.apply(&mut object, 1, 2);
    }
}
