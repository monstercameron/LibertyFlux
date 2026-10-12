//! Conditional float stores: a float field of an object is replaced only when the incoming value differs from it.
//!
//! Four small routines of the original belong to one task family. Each is a method of a task object that takes
//! one incoming float and writes it into a float field of the object, but only when the field's current value
//! differs from the incoming one. The field offset is the only thing that differs between the routines.
//! [`FloatIfChanged`] describes one of them as data, and [`FloatIfChanged::set`] is the routine.
//!
//! Floating-point values are passed as their 32 bits, so the lift never rounds or converts anything. The
//! comparison is the floating-point comparison of the two values: a not-a-number differs from every value,
//! itself included, and positive zero equals negative zero. So a field holding positive zero keeps it when the
//! incoming value is negative zero, and a field holding a not-a-number is always overwritten.
//!
//! Specification of [`FloatIfChanged::set`]:
//!
//! - The field's current word is read as a float.
//! - When that float is not equal to the incoming float, the incoming bits are written to the field, as four
//!   bytes least significant byte first. The bits are copied unchanged, including the sign and payload of a
//!   not-a-number.
//! - When the two floats are equal, nothing is written.
//! - Nothing else of the object is read or written, and nothing here loops or allocates.
//! - A field that lies outside the object is a caller error and panics.

use crate::field_store::ObjectBytes;

/// One routine of this family: the offset of its float field in the object.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FloatIfChanged {
    /// Byte offset of the float field from the start of the object.
    pub offset: usize,
}

impl FloatIfChanged {
    /// Describes one routine by the offset of its float field.
    #[must_use]
    pub const fn new(offset: usize) -> Self {
        Self { offset }
    }

    /// Replaces the float field with `incoming`, given as its bits, when the field's current float differs from
    /// it. See the module specification.
    ///
    /// # Panics
    ///
    /// When the field does not lie inside the object.
    pub fn set(&self, object: &mut ObjectBytes, incoming: u32) {
        let stored = f32::from_bits(object.word(self.offset));
        if stored != f32::from_bits(incoming) {
            object.write(self.offset, &incoming.to_le_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FloatIfChanged;
    use crate::field_store::ObjectBytes;

    fn object_with(offset: usize, bits: u32) -> ObjectBytes {
        let mut object = ObjectBytes::zeroed(0x100);
        object.write(offset, &bits.to_le_bytes());
        object
    }

    #[test]
    fn a_different_value_is_written() {
        let routine = FloatIfChanged::new(0x10);
        let mut object = object_with(0x10, 1.0_f32.to_bits());
        routine.set(&mut object, 2.5_f32.to_bits());
        assert_eq!(object.word(0x10), 2.5_f32.to_bits());
    }

    #[test]
    fn an_equal_value_is_not_written() {
        let routine = FloatIfChanged::new(0x10);
        let mut object = object_with(0x10, 0.0_f32.to_bits());
        routine.set(&mut object, (-0.0_f32).to_bits());
        assert_eq!(object.word(0x10), 0.0_f32.to_bits());
    }

    #[test]
    fn a_not_a_number_is_always_written() {
        let routine = FloatIfChanged::new(0x0);
        let quiet = 0x7FC0_1234;
        let mut object = object_with(0x0, quiet);
        routine.set(&mut object, quiet);
        assert_eq!(object.word(0x0), quiet);
        let mut other = object_with(0x0, 3.0_f32.to_bits());
        routine.set(&mut other, quiet);
        assert_eq!(other.word(0x0), quiet);
    }

    #[test]
    fn only_the_field_changes() {
        let routine = FloatIfChanged::new(0x4);
        let mut object = object_with(0x4, 1.0_f32.to_bits());
        object.write(0x0, &[0xAB; 4]);
        routine.set(&mut object, 9.0_f32.to_bits());
        assert_eq!(object.word(0x0), 0xABAB_ABAB);
        assert_eq!(object.word(0x4), 9.0_f32.to_bits());
    }
}
