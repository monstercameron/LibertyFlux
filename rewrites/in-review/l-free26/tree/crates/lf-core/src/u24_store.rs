//! Three-byte stores: the low 24 bits of a parameter written into three consecutive bytes of an object.
//!
//! Four small routines of the original each take an object and one 32-bit parameter, write the parameter's
//! low three bytes into three consecutive byte fields, and return the parameter shifted right by eight. The
//! field offset is the only thing that differs between them. [`U24Store`] describes one of them as data, and
//! [`U24Store::store`] is the routine.
//!
//! Specification of [`U24Store::store`]:
//!
//! - The byte at the offset receives the parameter's least significant byte.
//! - The byte one past the offset receives the next byte of the parameter, and the byte two past the offset
//!   receives the third. The parameter's top byte is never written.
//! - The return value is the parameter shifted right by eight bits, with the top byte of the result zero.
//! - Nothing else of the object is read or written, and nothing here loops or allocates.
//! - A field that lies outside the object is a caller error and panics.

use crate::field_store::ObjectBytes;

/// One routine of this family: the offset of its lowest byte in the object.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct U24Store {
    /// Byte offset of the least significant of the three bytes, from the start of the object.
    pub offset: usize,
}

impl U24Store {
    /// Describes one routine by the offset of its lowest byte.
    #[must_use]
    pub const fn new(offset: usize) -> Self {
        Self { offset }
    }

    /// Writes the low three bytes of `value` into the object at [`Self::offset`], least significant byte first,
    /// and returns `value >> 8`. See the module specification.
    ///
    /// # Panics
    ///
    /// When the three bytes do not lie inside the object.
    pub fn store(&self, object: &mut ObjectBytes, value: u32) -> u32 {
        object.write(self.offset, &value.to_le_bytes()[..3]);
        value >> 8
    }
}

#[cfg(test)]
mod tests {
    use super::U24Store;
    use crate::field_store::ObjectBytes;

    #[test]
    fn three_bytes_least_significant_first() {
        let routine = U24Store::new(0x5);
        let mut object = ObjectBytes::zeroed(0x10);
        let returned = routine.store(&mut object, 0xAABB_CCDD);
        assert_eq!(returned, 0x00AA_BBCC);
        assert_eq!(object.byte(0x4), 0);
        assert_eq!(object.byte(0x5), 0xDD);
        assert_eq!(object.byte(0x6), 0xCC);
        assert_eq!(object.byte(0x7), 0xBB);
        assert_eq!(object.byte(0x8), 0);
    }

    #[test]
    fn the_top_byte_of_the_parameter_is_not_stored() {
        let routine = U24Store::new(0x0);
        let mut object = ObjectBytes::from_bytes(&[0x11; 4]);
        routine.store(&mut object, 0xFF00_0000);
        assert_eq!(object.bytes(), &[0, 0, 0, 0x11]);
    }

    #[test]
    #[should_panic]
    fn bytes_past_the_object_panic() {
        let routine = U24Store::new(0x2);
        let mut object = ObjectBytes::zeroed(0x4);
        routine.store(&mut object, 1);
    }
}
