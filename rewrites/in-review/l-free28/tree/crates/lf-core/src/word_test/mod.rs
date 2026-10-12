//! Word tests: a comparison of one word with a constant or with a parameter, answered as 0 or 1.
//!
//! Several small routines of the original answer a yes-or-no question about one word: they read a word of
//! their object or a static word, compare it with a constant or with a parameter, and return 1 when the
//! comparison holds and 0 when it does not. [`Predicate`] describes one such routine as data: where the word
//! is, the comparison, and the right-hand side. [`Predicate::evaluate`] is the routine. The instances the
//! verified rewrites were checked against are listed in [`instances`].
//!
//! Specification of [`Predicate::evaluate`]:
//!
//! - The tested word is read from the object's field (a 32-bit word at the field's offset) or from the
//!   static word table, and nothing else is read.
//! - [`Compare::Equal`] and [`Compare::NotEqual`] compare the bit patterns of the words.
//! - [`Compare::SignedGreater`] compares the words as signed 32-bit integers: the tested word is greater
//!   than the right-hand side.
//! - The right-hand side is the constant of the instance, or the parameter passed to the routine.
//! - The result is 1 when the comparison holds and 0 when it does not.
//! - A field or word outside the table is a caller error and panics.
//!
//! Nothing here loops or allocates per call.

use crate::field_store::ObjectBytes;
use crate::static_list::{StaticWords, WordId};

pub mod instances;

/// The comparison a predicate makes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare {
    /// The tested word equals the right-hand side.
    Equal,
    /// The tested word differs from the right-hand side.
    NotEqual,
    /// The tested word, read as a signed integer, is greater than the right-hand side read the same way.
    SignedGreater,
}

/// The right-hand side of a comparison.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rhs {
    /// A constant of the instance.
    Constant(u32),
    /// The parameter the routine receives.
    Parameter,
}

/// Where the tested word is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    /// A 32-bit word of the object, at this byte offset.
    Field(usize),
    /// A static word, by its number in the table.
    Global(WordId),
}

/// One word test, as data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Predicate {
    /// Where the tested word is.
    pub source: Source,
    /// The comparison.
    pub compare: Compare,
    /// The right-hand side.
    pub rhs: Rhs,
}

impl Predicate {
    /// Answers the question for one object, one table of static words and one parameter: 1 when the
    /// comparison holds, 0 when it does not.
    ///
    /// # Panics
    ///
    /// When the field or the static word lies outside its table.
    #[must_use]
    pub fn evaluate(&self, object: &ObjectBytes, globals: &StaticWords, parameter: u32) -> u32 {
        let tested = match self.source {
            Source::Field(offset) => object.word(offset),
            Source::Global(id) => globals.word(id),
        };
        let rhs = match self.rhs {
            Rhs::Constant(value) => value,
            Rhs::Parameter => parameter,
        };
        let holds = match self.compare {
            Compare::Equal => tested == rhs,
            Compare::NotEqual => tested != rhs,
            Compare::SignedGreater => (tested as i32) > (rhs as i32),
        };
        u32::from(holds)
    }
}

#[cfg(test)]
mod tests {
    use super::{Compare, Predicate, Rhs, Source};
    use crate::field_store::ObjectBytes;
    use crate::static_list::{StaticWords, WordId};

    fn field_test(compare: Compare, rhs: Rhs) -> Predicate {
        Predicate {
            source: Source::Field(4),
            compare,
            rhs,
        }
    }

    #[test]
    fn equal_constant_holds_only_for_that_constant() {
        let mut object = ObjectBytes::zeroed(8);
        let globals = StaticWords::new(0);
        let test = field_test(Compare::Equal, Rhs::Constant(2));
        assert_eq!(test.evaluate(&object, &globals, 0), 0);
        object = ObjectBytes::from_bytes(&[0, 0, 0, 0, 2, 0, 0, 0]);
        assert_eq!(test.evaluate(&object, &globals, 0), 1);
    }

    #[test]
    fn not_equal_constant_is_the_negation() {
        let object = ObjectBytes::from_bytes(&[0, 0, 0, 0, 200, 0, 0, 0]);
        let globals = StaticWords::new(0);
        let test = field_test(Compare::NotEqual, Rhs::Constant(200));
        assert_eq!(test.evaluate(&object, &globals, 0), 0);
        let other = ObjectBytes::zeroed(8);
        assert_eq!(test.evaluate(&other, &globals, 0), 1);
    }

    #[test]
    fn parameter_right_hand_side_is_compared_with_the_field() {
        let object = ObjectBytes::from_bytes(&[0, 0, 0, 0, 9, 0, 0, 0]);
        let globals = StaticWords::new(0);
        let test = field_test(Compare::Equal, Rhs::Parameter);
        assert_eq!(test.evaluate(&object, &globals, 9), 1);
        assert_eq!(test.evaluate(&object, &globals, 8), 0);
    }

    #[test]
    fn signed_greater_treats_the_top_bit_as_a_sign() {
        let mut globals = StaticWords::new(1);
        let test = Predicate {
            source: Source::Global(WordId(0)),
            compare: Compare::SignedGreater,
            rhs: Rhs::Constant(2),
        };
        let object = ObjectBytes::zeroed(0);
        globals.set_word(WordId(0), 3);
        assert_eq!(test.evaluate(&object, &globals, 0), 1);
        globals.set_word(WordId(0), 2);
        assert_eq!(test.evaluate(&object, &globals, 0), 0);
        globals.set_word(WordId(0), u32::MAX);
        assert_eq!(test.evaluate(&object, &globals, 0), 0);
    }
}
