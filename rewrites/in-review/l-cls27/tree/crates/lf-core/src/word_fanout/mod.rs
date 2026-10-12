//! Copies of one word into several words of a table.
//!
//! Some routines of the original read one word of the game's globals and
//! store its value, unchanged, into a fixed set of other words. The set is
//! fixed per routine, the source is fixed per routine, and the routine returns
//! zero. [`WordFanout`] holds one such routine: the index of its source word
//! and the indices of its targets, in the order the routine writes them.
//!
//! The table is a slice of words that the caller owns. The lift never names
//! an address: the caller maps each global to its index in the table.
//!
//! Specification of [`WordFanout::apply`]:
//!
//! - The value of the source word is read once, before any write.
//! - Each target is then written with that value, in the order the targets
//!   are listed. A target listed twice is written twice.
//! - Nothing else is written, and the routine's result is always zero, which
//!   is why [`WordFanout::apply`] returns nothing and the caller supplies
//!   the zero.
//!
//! [`SOURCE_TO_THREE`] is the shape the six lifted routines share: a source
//! word and three targets. Their table indices are the caller's own layout and
//! carry no addresses.

/// One routine that copies its source word into each of its target words.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WordFanout<'a> {
    /// Index of the word whose value is copied.
    pub source: usize,
    /// Indices of the words written with that value, in order.
    pub targets: &'a [usize],
}

impl<'a> WordFanout<'a> {
    /// A fan-out from `source` to `targets`.
    #[must_use]
    pub const fn new(source: usize, targets: &'a [usize]) -> Self {
        Self { source, targets }
    }

    /// Writes the value of the source word into every target, in order.
    ///
    /// # Panics
    ///
    /// When the source or a target is outside `words`. The original would
    /// write outside the set of globals the routine names; the lift stops
    /// instead.
    pub fn apply(&self, words: &mut [u32]) {
        let value = words[self.source];
        for &target in self.targets {
            words[target] = value;
        }
    }
}

/// The shape of the lifted family: one source word and three targets. Each
/// rewrite of the family has this shape over its own globals; the caller maps
/// each global to its index in the table, and the test crate `lf-fanoutdiff`
/// keeps that mapping for every rewrite.
pub const SOURCE_TO_THREE: WordFanout<'static> = WordFanout::new(0, &[1, 2, 3]);

#[cfg(test)]
mod tests {
    use super::{SOURCE_TO_THREE, WordFanout};

    #[test]
    fn every_target_gets_the_source_value() {
        let mut words = [0u32, 0, 0, 0];
        WordFanout::new(0, &[1, 2, 3]).apply(&mut words);
        words[0] = 0xDEAD_BEEF;
        WordFanout::new(0, &[1, 2, 3]).apply(&mut words);
        assert_eq!(words, [0xDEAD_BEEF; 4]);
    }

    #[test]
    fn the_source_is_read_before_any_write() {
        let mut words = [7u32, 0];
        WordFanout::new(0, &[0, 1]).apply(&mut words);
        assert_eq!(words, [7, 7]);
    }

    #[test]
    fn a_word_outside_the_table_stops_the_lift() {
        let result = std::panic::catch_unwind(|| {
            let mut words = [0u32; 2];
            WordFanout::new(0, &[5]).apply(&mut words);
        });
        assert!(result.is_err());
    }

    #[test]
    fn the_family_shape_has_three_distinct_targets_apart_from_the_source() {
        let fan = SOURCE_TO_THREE;
        assert_eq!(fan.targets.len(), 3);
        assert!(fan.targets.iter().all(|&t| t != fan.source));
        assert!(fan.targets[0] != fan.targets[1] && fan.targets[1] != fan.targets[2]);
    }
}
