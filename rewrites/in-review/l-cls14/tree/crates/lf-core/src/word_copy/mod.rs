//! Word copies: one static word's value copied into another word.
//!
//! Several small routines of the original do exactly one thing: they read one static word and write its
//! value into another word, and return zero. [`WordCopy`] describes one such routine as data: the word it
//! reads and the word it writes. [`WordCopy::apply`] is the routine. The instances the verified rewrites were
//! checked against are listed in [`instances`].
//!
//! Specification of [`WordCopy::apply`]:
//!
//! - The value of the source word is read first and then written to the destination word. When the two
//!   words are the same word, the value is unchanged.
//! - No other word is read or written.
//! - The result is always zero: the routines return zero whatever the copy did.
//! - An id outside the table is a caller error and panics, as for every [`StaticWords`] access.
//!
//! Floating-point words are copied as their 32 bits. No arithmetic is done, so no value changes.

use crate::static_list::{StaticWords, WordId};

pub mod instances;

/// One word copy, as data: the word it reads and the word it writes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WordCopy {
    /// The word whose value is copied.
    pub source: WordId,
    /// The word that receives the value.
    pub destination: WordId,
}

impl WordCopy {
    /// Copies the source word's value into the destination word and returns zero.
    ///
    /// # Panics
    ///
    /// When either id is outside the table.
    pub fn apply(&self, words: &mut StaticWords) -> u32 {
        let value = words.word(self.source);
        words.set_word(self.destination, value);
        0
    }
}

#[cfg(test)]
mod tests {
    use super::WordCopy;
    use crate::static_list::{StaticWords, WordId};

    #[test]
    fn copies_the_source_value_and_nothing_else() {
        let mut words = StaticWords::new(3);
        words.set_word(WordId(0), 0xABCD_0123);
        words.set_word(WordId(2), 7);
        let copy = WordCopy {
            source: WordId(0),
            destination: WordId(1),
        };
        assert_eq!(copy.apply(&mut words), 0);
        assert_eq!(words.word(WordId(0)), 0xABCD_0123);
        assert_eq!(words.word(WordId(1)), 0xABCD_0123);
        assert_eq!(words.word(WordId(2)), 7);
    }

    #[test]
    fn copying_a_word_onto_itself_changes_nothing() {
        let mut words = StaticWords::new(1);
        words.set_word(WordId(0), 5);
        let copy = WordCopy {
            source: WordId(0),
            destination: WordId(0),
        };
        assert_eq!(copy.apply(&mut words), 0);
        assert_eq!(words.word(WordId(0)), 5);
    }

    #[test]
    #[should_panic]
    fn an_id_outside_the_table_panics() {
        let mut words = StaticWords::new(1);
        let copy = WordCopy {
            source: WordId(0),
            destination: WordId(4),
        };
        let _ = copy.apply(&mut words);
    }
}
