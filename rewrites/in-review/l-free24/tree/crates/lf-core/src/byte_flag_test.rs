//! Byte-flag tests: a yes-or-no answer about whether a byte field of an object has any bit of a mask set.
//!
//! Several small routines of the original read one byte of their object and answer 1 when that byte has at
//! least one bit of a fixed mask set, and 0 when none does. The byte's offset and the mask are the only things
//! that differ between the routines. [`ByteFlagTest`] holds one such routine as data, and
//! [`ByteFlagTest::evaluate`] is the routine. A test on a single bit is a mask with one bit set.
//!
//! Specification of [`ByteFlagTest::evaluate`]:
//!
//! - The tested byte is read at the field's offset in the object, and nothing else is read.
//! - The answer is 1 when `byte & mask` is not zero, and 0 when it is zero.
//! - Nothing is written, and nothing here loops or allocates.
//! - A field outside the object is a caller error and panics.

use crate::field_store::ObjectBytes;

/// One byte-flag test, as data: the byte's offset in the object and the mask it is tested against.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ByteFlagTest {
    /// Byte offset of the tested byte from the start of the object.
    pub offset: usize,
    /// The bits tested: the answer is yes when the byte has at least one of them set.
    pub mask: u8,
}

impl ByteFlagTest {
    /// Answers the question for one object: 1 when the tested byte has a bit of the mask set, 0 when not.
    ///
    /// # Panics
    ///
    /// When the tested byte lies outside the object.
    #[must_use]
    pub fn evaluate(&self, object: &ObjectBytes) -> u32 {
        u32::from(object.byte(self.offset) & self.mask != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::ByteFlagTest;
    use crate::field_store::ObjectBytes;

    #[test]
    fn a_single_bit_mask_answers_for_that_bit_only() {
        let test = ByteFlagTest {
            offset: 1,
            mask: 0x10,
        };
        let set = ObjectBytes::from_bytes(&[0, 0x10, 0]);
        let clear = ObjectBytes::from_bytes(&[0x10, 0xEF, 0x10]);
        assert_eq!(test.evaluate(&set), 1);
        assert_eq!(test.evaluate(&clear), 0);
    }

    #[test]
    fn any_bit_of_a_multi_bit_mask_is_enough() {
        let test = ByteFlagTest {
            offset: 0,
            mask: 3,
        };
        assert_eq!(test.evaluate(&ObjectBytes::from_bytes(&[2])), 1);
        assert_eq!(test.evaluate(&ObjectBytes::from_bytes(&[1])), 1);
        assert_eq!(test.evaluate(&ObjectBytes::from_bytes(&[4])), 0);
    }

    #[test]
    fn a_zero_byte_answers_no_for_every_mask() {
        for mask in [0x01u8, 0x40, 0xFF] {
            let test = ByteFlagTest { offset: 0, mask };
            assert_eq!(test.evaluate(&ObjectBytes::zeroed(1)), 0);
        }
    }

    #[test]
    #[should_panic]
    fn a_field_past_the_end_panics() {
        let test = ByteFlagTest {
            offset: 4,
            mask: 1,
        };
        let _ = test.evaluate(&ObjectBytes::zeroed(4));
    }
}
