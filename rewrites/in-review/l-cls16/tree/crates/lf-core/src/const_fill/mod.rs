//! Storing one constant into several words of a table.
//!
//! Some routines of the original store one fixed constant, unchanged, into a
//! fixed set of globals, and return zero. Each routine has its own constant and
//! its own set of words. [`WordFill`] holds the constant of one such routine,
//! and [`WordFill::apply`] writes it into the words the caller names.
//!
//! The table is a slice of words that the caller owns, and the caller maps each
//! global to its index. No address appears here.
//!
//! Specification of [`WordFill::apply`]:
//!
//! - Each target, in the order the targets are listed, is written with the
//!   constant. A target listed twice is written twice.
//! - Nothing else is written. The routine's result is always zero, which the
//!   caller supplies.
//!
//! [`FILL_VALUES`] lists the constants of the routines lifted from verified
//! rewrites.

/// One routine that writes its constant into each of its target words.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WordFill {
    /// The constant written into every target.
    pub value: u32,
}

impl WordFill {
    /// A fill with the given constant.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self { value }
    }

    /// Writes the constant into each target word, in order.
    ///
    /// # Panics
    ///
    /// When a target is outside `words`. The original would write outside the
    /// globals the routine names; the lift stops instead.
    pub fn apply(&self, words: &mut [u32], targets: &[usize]) {
        for &target in targets {
            words[target] = self.value;
        }
    }
}

/// The constants of the four fills lifted from verified rewrites: minus one for
/// the timing identifiers, zero for the three others.
pub const FILL_VALUES: [u32; 4] = [0xFFFF_FFFF, 0, 0, 0];

#[cfg(test)]
mod tests {
    use super::{FILL_VALUES, WordFill};

    #[test]
    fn every_target_gets_the_constant_and_nothing_else_changes() {
        let mut words = [5u32, 6, 7, 8];
        WordFill::new(0).apply(&mut words, &[0, 2]);
        assert_eq!(words, [0, 6, 0, 8]);
    }

    #[test]
    fn a_target_listed_twice_is_written_twice_with_the_same_value() {
        let mut words = [1u32, 2];
        WordFill::new(9).apply(&mut words, &[1, 1]);
        assert_eq!(words, [1, 9]);
    }

    #[test]
    fn a_target_outside_the_table_stops_the_lift() {
        let result = std::panic::catch_unwind(|| {
            let mut words = [0u32; 2];
            WordFill::new(1).apply(&mut words, &[2]);
        });
        assert!(result.is_err());
    }

    #[test]
    fn the_table_has_one_minus_one_and_three_zeros() {
        assert_eq!(FILL_VALUES.iter().filter(|&&v| v == 0xFFFF_FFFF).count(), 1);
        assert_eq!(FILL_VALUES.iter().filter(|&&v| v == 0).count(), 3);
    }
}
