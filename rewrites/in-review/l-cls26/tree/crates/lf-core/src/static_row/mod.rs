//! Static rows: four consecutive words written from two constants.
//!
//! Several small routines of the original do the same thing to a block of four
//! words in a fixed place in the executable's data: the first word receives a
//! constant (a code address or a data address, relocated when the image is
//! loaded), the second and third words receive zero, and the fourth receives a
//! second constant. Some routines write the block as four 32-bit stores, and
//! some as two 64-bit stores of the same four words (the low half of the first
//! store is the first constant, and the high half of the second store is the
//! second constant). The words written are the same in both forms.
//!
//! [`StaticRow`] holds the two constants of one routine. [`StaticRow::write`]
//! writes the block; the caller names the block by the index of its first word
//! in a [`StaticWords`] table, so no address appears here and nothing depends
//! on pointer width.
//!
//! Specification of [`StaticRow::write`]:
//!
//! - The four words of the block are written in order: the first word gets
//!   [`StaticRow::first`], the second and third words get zero, and the fourth
//!   word gets [`StaticRow::second`].
//! - Nothing else is written. The words before and after the block keep their
//!   values.
//! - A block that does not lie inside the table is a caller error and panics.
//!   The original would have written outside the block the routine names; the
//!   lift stops instead.
//!
//! The verified routines return zero, or return nothing at all. The lifted
//! method returns nothing: a caller that must return zero supplies it.
//!
//! The routines this method was checked against are listed by the test crate
//! `lf-25-rowdiff`, which runs the method against each verified rewrite on
//! generated states of the words, and compares every word.

use crate::static_list::{StaticWords, WordId};

/// Offset of the first constant within the block.
pub const FIRST_OFFSET: usize = 0;

/// Offsets of the two zero words within the block.
pub const ZERO_OFFSETS: [usize; 2] = [1, 2];

/// Offset of the second constant within the block.
pub const SECOND_OFFSET: usize = 3;

/// Number of words in one block.
pub const BLOCK_WORDS: usize = 4;

/// The two constants of one static row.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StaticRow {
    /// The constant written into the first word of the block.
    pub first: u32,
    /// The constant written into the fourth word of the block.
    pub second: u32,
}

impl StaticRow {
    /// A row with the given constants.
    #[must_use]
    pub const fn new(first: u32, second: u32) -> Self {
        Self { first, second }
    }

    /// Writes the four-word block whose first word is `base`: the first
    /// constant, two zeros and the second constant, in that order. See the
    /// module specification.
    ///
    /// # Panics
    ///
    /// When any word of the block is outside `words`.
    pub fn write(&self, words: &mut StaticWords, base: WordId) {
        words.set_word(WordId(base.0 + FIRST_OFFSET), self.first);
        for offset in ZERO_OFFSETS {
            words.set_word(WordId(base.0 + offset), 0);
        }
        words.set_word(WordId(base.0 + SECOND_OFFSET), self.second);
    }
}

#[cfg(test)]
mod tests {
    use super::{BLOCK_WORDS, StaticRow};
    use crate::static_list::{StaticWords, WordId};

    #[test]
    fn the_block_gets_the_two_constants_and_two_zeros() {
        let mut words = StaticWords::new(8);
        for k in 0..8 {
            words.set_word(WordId(k), 0xFFFF_FFFF);
        }
        StaticRow::new(0x0043_EA90, 0x0040_9610).write(&mut words, WordId(2));
        let got: Vec<u32> = (0..8).map(|k| words.word(WordId(k))).collect();
        assert_eq!(
            got,
            vec![
                0xFFFF_FFFF,
                0xFFFF_FFFF,
                0x0043_EA90,
                0,
                0,
                0x0040_9610,
                0xFFFF_FFFF,
                0xFFFF_FFFF
            ]
        );
    }

    #[test]
    fn a_block_at_the_end_of_the_table_is_written() {
        let mut words = StaticWords::new(BLOCK_WORDS);
        StaticRow::new(7, 9).write(&mut words, WordId(0));
        assert_eq!(
            (0..4).map(|k| words.word(WordId(k))).collect::<Vec<_>>(),
            vec![7, 0, 0, 9]
        );
    }

    #[test]
    #[should_panic]
    fn a_block_past_the_end_of_the_table_panics() {
        let mut words = StaticWords::new(BLOCK_WORDS);
        StaticRow::new(1, 2).write(&mut words, WordId(1));
    }

    #[test]
    fn writing_twice_leaves_the_second_write() {
        let mut words = StaticWords::new(4);
        StaticRow::new(1, 2).write(&mut words, WordId(0));
        StaticRow::new(3, 4).write(&mut words, WordId(0));
        assert_eq!(
            (0..4).map(|k| words.word(WordId(k))).collect::<Vec<_>>(),
            vec![3, 0, 0, 4]
        );
    }
}
