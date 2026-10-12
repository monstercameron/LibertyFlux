//! Field comparisons: one field of an object compared with a constant, answered as 0 or 1.
//!
//! Several small routines of the original read one field of their object, read it as a 32-bit float or as a
//! signed 32-bit integer, compare it with a constant, and answer 1 when the comparison holds. Some answer the
//! negation. The routines differ in the field, its type, the comparison and the constant. [`FieldCompare`]
//! holds one such routine as data, and [`FieldCompare::evaluate`] is the routine.
//!
//! Specification of [`FieldCompare::evaluate`]:
//!
//! - The field is read at its offset as four bytes, least significant first. For [`Operand::Float`] those bits
//!   are a 32-bit float; for [`Operand::Signed`] they are a signed 32-bit integer.
//! - The comparison is made between the field and the constant, with the usual rules of the type: a float
//!   comparison with a not-a-number is false for every operator except inequality.
//! - The answer is 1 when the comparison holds, or when it does not and `negate` is set; otherwise 0.
//! - Nothing is written, and nothing here loops or allocates.
//! - A field outside the object is a caller error and panics.

use crate::field_store::ObjectBytes;

/// The type a field is read as.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operand {
    /// A 32-bit float, compared with the float rules.
    Float,
    /// A signed 32-bit integer.
    Signed,
}

/// The comparison a field makes with its constant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare {
    /// The field is less than the constant.
    Less,
    /// The field is at most the constant.
    LessEqual,
    /// The field is greater than the constant.
    Greater,
    /// The field is at least the constant.
    GreaterEqual,
}

/// One field comparison, as data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldCompare {
    /// Byte offset of the field from the start of the object.
    pub offset: usize,
    /// The type the field is read as.
    pub operand: Operand,
    /// The comparison.
    pub compare: Compare,
    /// The constant, as its 32 bits: the bits of a float, or a signed integer.
    pub constant: u32,
    /// Whether the answer is the negation of the comparison.
    pub negate: bool,
}

impl FieldCompare {
    /// Answers the question for one object: 1 or 0.
    ///
    /// # Panics
    ///
    /// When the field lies outside the object.
    #[must_use]
    pub fn evaluate(&self, object: &ObjectBytes) -> u32 {
        let bits = object.word(self.offset);
        let holds = match self.operand {
            Operand::Float => {
                let field = f32::from_bits(bits);
                let constant = f32::from_bits(self.constant);
                match self.compare {
                    Compare::Less => field < constant,
                    Compare::LessEqual => field <= constant,
                    Compare::Greater => field > constant,
                    Compare::GreaterEqual => field >= constant,
                }
            }
            Operand::Signed => {
                let field = bits as i32;
                let constant = self.constant as i32;
                match self.compare {
                    Compare::Less => field < constant,
                    Compare::LessEqual => field <= constant,
                    Compare::Greater => field > constant,
                    Compare::GreaterEqual => field >= constant,
                }
            }
        };
        u32::from(holds != self.negate)
    }
}

#[cfg(test)]
mod tests {
    use super::{Compare, FieldCompare, Operand};
    use crate::field_store::ObjectBytes;

    fn object_with(bits: u32) -> ObjectBytes {
        let mut object = ObjectBytes::zeroed(8);
        object.write(4, &bits.to_le_bytes());
        object
    }

    #[test]
    fn a_float_greater_than_zero() {
        let test = FieldCompare {
            offset: 4,
            operand: Operand::Float,
            compare: Compare::Greater,
            constant: 0,
            negate: false,
        };
        assert_eq!(test.evaluate(&object_with(1.5f32.to_bits())), 1);
        assert_eq!(test.evaluate(&object_with((-1.5f32).to_bits())), 0);
        assert_eq!(test.evaluate(&object_with(0)), 0);
    }

    #[test]
    fn a_not_a_number_fails_every_ordering_test() {
        let nan = f32::NAN.to_bits();
        let greater = FieldCompare {
            offset: 4,
            operand: Operand::Float,
            compare: Compare::Greater,
            constant: 0,
            negate: false,
        };
        assert_eq!(greater.evaluate(&object_with(nan)), 0);
        let negated = FieldCompare { negate: true, ..greater };
        assert_eq!(negated.evaluate(&object_with(nan)), 1);
    }

    #[test]
    fn a_signed_field_below_zero_has_its_sign_bit_set() {
        let test = FieldCompare {
            offset: 4,
            operand: Operand::Signed,
            compare: Compare::Less,
            constant: 0,
            negate: false,
        };
        assert_eq!(test.evaluate(&object_with(0xFFFF_FFFF)), 1);
        assert_eq!(test.evaluate(&object_with(0x7FFF_FFFF)), 0);
    }

    #[test]
    fn a_float_at_least_one() {
        let test = FieldCompare {
            offset: 4,
            operand: Operand::Float,
            compare: Compare::GreaterEqual,
            constant: 1.0f32.to_bits(),
            negate: false,
        };
        assert_eq!(test.evaluate(&object_with(1.0f32.to_bits())), 1);
        assert_eq!(test.evaluate(&object_with(0.999_999_9f32.to_bits())), 0);
    }
}
