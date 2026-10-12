//! Packed low-byte flags: a word whose upper three bytes are kept and whose low byte is set to a yes-or-no answer.
//!
//! Several small routines of the original return a word that keeps the upper three bytes of a value and
//! replaces the low byte with 1 or 0, the answer to a question about the value. The routines differ in where
//! the value comes from (a parameter, or a word of their object), in whether the value is first moved by a
//! constant base, and in the question asked. [`PackedFlag`] holds one such routine as data, and
//! [`PackedFlag::apply`] is the routine on a value.
//!
//! Specification of [`PackedFlag::apply`]:
//!
//! - The value is `word - base` in 32-bit wrapping arithmetic when the routine subtracts a base, and the word
//!   itself otherwise. [`Condition::AtMost`] tests that value; the other conditions test the word itself.
//! - The result is the value with its low byte cleared, with bit zero set when the condition holds.
//!   The bit is set in the low byte only; the other seven bits of that byte are zero.
//! - [`PackedFlag::read`] takes the word from its object field first. Nothing is written.

use crate::field_store::ObjectBytes;

/// The question a packed flag asks of its value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Condition {
    /// The value is at most this limit, as unsigned numbers.
    AtMost(u32),
    /// The value equals one of the two constants.
    OneOf(u32, u32),
    /// The value has every bit of `set` and none of `clear`.
    SetAndClear {
        /// Bits that must be set.
        set: u32,
        /// Bits that must be clear.
        clear: u32,
    },
}

impl Condition {
    /// Whether the condition holds for `value`.
    #[must_use]
    pub fn holds(&self, value: u32) -> bool {
        match *self {
            Self::AtMost(limit) => value <= limit,
            Self::OneOf(first, second) => value == first || value == second,
            Self::SetAndClear { set, clear } => value & set != 0 && value & clear == 0,
        }
    }
}

/// Where the word comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    /// The routine's own parameter.
    Parameter,
    /// A 32-bit word of the object, at this byte offset.
    Field(usize),
}

/// One packed low-byte flag routine, as data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PackedFlag {
    /// Where the word comes from.
    pub source: Source,
    /// The base subtracted from the word before the condition is tested; zero when the routine subtracts none.
    pub base: u32,
    /// The question asked of the value.
    pub condition: Condition,
}

/// The low byte of a packed word.
const LOW_BYTE: u32 = 0xFF;

impl PackedFlag {
    /// The packed result for one value: its upper three bytes kept, its low byte set to the answer.
    ///
    /// The value is `word - base` when the routine has a base, and `word` otherwise. The condition is tested on
    /// the value, and the answer is 1 when it holds and 0 when it does not.
    #[must_use]
    pub fn apply(&self, word: u32) -> u32 {
        let value = word.wrapping_sub(self.base);
        let answer = u32::from(self.condition.holds(value));
        (value & !LOW_BYTE) | answer
    }

    /// The packed result for the word this routine reads from `object`, or for `parameter` when its source is
    /// the parameter.
    ///
    /// # Panics
    ///
    /// When the field lies outside the object.
    #[must_use]
    pub fn read(&self, object: &ObjectBytes, parameter: u32) -> u32 {
        let word = match self.source {
            Source::Parameter => parameter,
            Source::Field(offset) => object.word(offset),
        };
        self.apply(word)
    }
}

#[cfg(test)]
mod tests {
    use super::{Condition, PackedFlag, Source};
    use crate::field_store::ObjectBytes;

    #[test]
    fn a_limit_test_packs_the_shifted_value() {
        let flag = PackedFlag {
            source: Source::Parameter,
            base: 0x73,
            condition: Condition::AtMost(7),
        };
        assert_eq!(flag.apply(0x73), 0x0000_0001);
        assert_eq!(flag.apply(0x7A), 0x0000_0001);
        assert_eq!(flag.apply(0x7B), 0x0000_0000);
        assert_eq!(flag.apply(0x72), 0xFFFF_FFFF & !0xFF);
    }

    #[test]
    fn two_constants_answer_yes_for_either() {
        let flag = PackedFlag {
            source: Source::Field(4),
            base: 0,
            condition: Condition::OneOf(1, 3),
        };
        let mut object = ObjectBytes::zeroed(8);
        object.write(4, &3u32.to_le_bytes());
        assert_eq!(flag.read(&object, 0), 1);
        object.write(4, &2u32.to_le_bytes());
        assert_eq!(flag.read(&object, 0), 0);
    }

    #[test]
    fn a_parameter_source_ignores_the_object() {
        let flag = PackedFlag {
            source: Source::Parameter,
            base: 0,
            condition: Condition::OneOf(1, 2),
        };
        let object = ObjectBytes::zeroed(4);
        assert_eq!(flag.read(&object, 2), 1);
        assert_eq!(flag.read(&object, 5), 0);
    }

    #[test]
    fn a_set_and_clear_test_needs_both_halves() {
        let condition = Condition::SetAndClear {
            set: 0x10,
            clear: 0x20,
        };
        assert!(condition.holds(0x10));
        assert!(!condition.holds(0x30));
        assert!(!condition.holds(0x00));
    }

    #[test]
    fn the_upper_bits_are_kept_and_only_the_low_byte_changes() {
        let flag = PackedFlag {
            source: Source::Parameter,
            base: 0,
            condition: Condition::AtMost(0),
        };
        assert_eq!(flag.apply(0xABCD_EF55), 0xABCD_EF00);
        assert_eq!(flag.apply(0xABCD_EF00), 0xABCD_EF01);
    }
}
