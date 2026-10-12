//! Yes-or-no tests over several fields of one object.
//!
//! Several small routines of the original look at a few fixed fields of an
//! object and answer one, if one of them is nonzero, if all of them are, or if
//! all of them are zero. A field is a little-endian word or byte at a fixed
//! offset. The routines differ in the offsets, the field width and the
//! combination, and that is what [`FieldPredicate`] holds.
//!
//! Specification of [`FieldPredicate::evaluate`]:
//!
//! - Each field is read at its offset in the object, as an unsigned value of
//!   the predicate's width, little endian.
//! - With [`Combine::Any`] the answer is yes when at least one field passes the
//!   [`Test`]; with [`Combine::All`] it is yes when every field passes it.
//! - The answer is returned as one for yes and zero for no.
//! - Nothing is written. The fields may be read in any order, and a field that
//!   the answer does not depend on may be left unread; the result is the same.
//!
//! [`PREDICATES`] is not a table of addresses: the offsets are the object's own
//! layout. It lists the predicates lifted from verified rewrites.

/// How the fields combine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Combine {
    /// Yes when at least one field passes the test.
    Any,
    /// Yes when every field passes the test.
    All,
}

/// The test each field must pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Test {
    /// The field is nonzero.
    NonZero,
    /// The field is zero.
    Zero,
}

/// The width of one field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Width {
    /// One byte.
    Byte,
    /// One little-endian 32-bit word.
    Word,
}

impl Width {
    /// The width in bytes.
    #[must_use]
    pub const fn bytes(self) -> usize {
        match self {
            Self::Byte => 1,
            Self::Word => 4,
        }
    }
}

/// One yes-or-no test over fields of one object.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FieldPredicate<'a> {
    /// Byte offsets of the fields in the object.
    pub offsets: &'a [usize],
    /// Width of each field.
    pub width: Width,
    /// How the fields combine.
    pub combine: Combine,
    /// The test each field must pass.
    pub test: Test,
}

impl FieldPredicate<'_> {
    /// The answer for `object`: one for yes, zero for no.
    ///
    /// # Panics
    ///
    /// When a field lies outside `object`. The original would read past the
    /// object; the lift stops instead.
    #[must_use]
    pub fn evaluate(&self, object: &[u8]) -> u32 {
        let passes = |offset: usize| {
            let value = read_field(object, offset, self.width);
            match self.test {
                Test::NonZero => value != 0,
                Test::Zero => value == 0,
            }
        };
        let yes = match self.combine {
            Combine::Any => self.offsets.iter().any(|&offset| passes(offset)),
            Combine::All => self.offsets.iter().all(|&offset| passes(offset)),
        };
        u32::from(yes)
    }
}

/// The unsigned value of a field of `width` at `offset`, little endian.
///
/// # Panics
///
/// When the field does not lie wholly inside `object`.
#[must_use]
pub fn read_field(object: &[u8], offset: usize, width: Width) -> u32 {
    let end = offset
        .checked_add(width.bytes())
        .filter(|&end| end <= object.len())
        .expect("a field lies outside the object");
    match width {
        Width::Byte => u32::from(object[offset]),
        Width::Word => {
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(&object[offset..end]);
            u32::from_le_bytes(bytes)
        }
    }
}

/// The predicates lifted from verified rewrites. Each entry is one rewrite's
/// fields and combination; the test crate `lf-predicatediff` keeps the mapping.
pub const PREDICATES: [FieldPredicate<'static>; 6] = [
    FieldPredicate { offsets: &[0x94, 0x98], width: Width::Word, combine: Combine::Any, test: Test::NonZero },
    FieldPredicate { offsets: &[0xB0, 0xB4], width: Width::Word, combine: Combine::Any, test: Test::NonZero },
    FieldPredicate { offsets: &[0x1268, 0x1269], width: Width::Byte, combine: Combine::Any, test: Test::NonZero },
    FieldPredicate { offsets: &[0x1190, 0x1191], width: Width::Byte, combine: Combine::All, test: Test::NonZero },
    FieldPredicate { offsets: &[0xFE0, 0xFE8], width: Width::Word, combine: Combine::All, test: Test::Zero },
    FieldPredicate { offsets: &[0x10, 0x14, 0x18], width: Width::Word, combine: Combine::All, test: Test::NonZero },
];

#[cfg(test)]
mod tests {
    use super::{Combine, FieldPredicate, PREDICATES, Test, Width, read_field};

    #[test]
    fn any_nonzero_word_fields() {
        let mut object = vec![0u8; 0x20];
        let predicate = FieldPredicate { offsets: &[0x08, 0x0c], width: Width::Word, combine: Combine::Any, test: Test::NonZero };
        assert_eq!(predicate.evaluate(&object), 0);
        object[0x0c] = 1;
        assert_eq!(predicate.evaluate(&object), 1);
    }

    #[test]
    fn all_nonzero_byte_fields() {
        let mut object = vec![0u8; 0x20];
        let predicate = FieldPredicate { offsets: &[4, 5], width: Width::Byte, combine: Combine::All, test: Test::NonZero };
        object[4] = 1;
        assert_eq!(predicate.evaluate(&object), 0);
        object[5] = 2;
        assert_eq!(predicate.evaluate(&object), 1);
    }

    #[test]
    fn all_zero_word_fields() {
        let mut object = vec![0u8; 0x20];
        let predicate = FieldPredicate { offsets: &[0x10, 0x18], width: Width::Word, combine: Combine::All, test: Test::Zero };
        assert_eq!(predicate.evaluate(&object), 1);
        object[0x18] = 0x80;
        assert_eq!(predicate.evaluate(&object), 0);
    }

    #[test]
    fn the_field_is_little_endian_and_unsigned() {
        let object = [0x78, 0x56, 0x34, 0xF2];
        assert_eq!(read_field(&object, 0, Width::Word), 0xF234_5678);
        assert_eq!(read_field(&object, 3, Width::Byte), 0xF2);
    }

    #[test]
    #[should_panic(expected = "outside the object")]
    fn a_field_past_the_end_stops_the_lift() {
        let object = vec![0u8; 6];
        let _ = read_field(&object, 4, Width::Word);
    }

    #[test]
    fn the_table_has_six_predicates() {
        assert_eq!(PREDICATES.len(), 6);
    }
}
