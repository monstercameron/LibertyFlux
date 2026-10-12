//! Cookie seal: stamping an object with a cookie derived from a global counter.
//!
//! Some objects of the original carry a cookie word. When such an object is
//! constructed, a routine stamps it: the cookie word is replaced by a value
//! that keeps the bits of the old cookie outside a mask and takes the bits of the
//! seal counter under the mask, the counter is advanced by one, the object's
//! vtable word is set, and one value field is set. The routine returns the
//! object itself.
//!
//! [`CookieSeal`] holds the constants of one routine: the marker and vtable words
//! it writes and the width of its value store. [`CookieSeal::seal`] is the
//! routine. The counter is the caller's word, passed by mutable reference; the
//! object is an [`ObjectBytes`] whose offsets are the object's byte offsets.
//!
//! Specification of [`CookieSeal::seal`], in order:
//!
//! 1. The counter's value is read, and the object's cookie word is read before
//!    anything is written.
//! 2. The vtable word receives the marker.
//! 3. The cookie word receives [`cookie`] of the old cookie and the counter.
//! 4. The counter is set to its value plus one, wrapping.
//! 5. The vtable word receives the vtable.
//! 6. The value field receives the value: all four bytes for
//!    [`ValueWidth::Word`], or the low byte only for [`ValueWidth::Byte`].
//!
//! Nothing else is written. The routine returns nothing; the object's own address,
//! which the original returns, is the caller's.

use crate::field_store::ObjectBytes;

/// Byte offset of the vtable word in the object.
pub const VTABLE_OFFSET: usize = 0;

/// Byte offset of the cookie word in the object.
pub const COOKIE_OFFSET: usize = 4;

/// Byte offset of the value field in the object.
pub const VALUE_OFFSET: usize = 8;

/// The bits of the counter that the cookie takes.
pub const COOKIE_MASK: u32 = 0x0000_3FFF;

/// The width of the value store.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ValueWidth {
    /// Four bytes, least significant first.
    Word,
    /// One byte: the low byte of the value.
    Byte,
}

/// The new cookie word for an object whose old cookie is `previous`, when the
/// counter reads `count`: the bits of `previous` outside [`COOKIE_MASK`] are kept,
/// and inside the mask the bits of `previous` are replaced by those of `count`.
#[must_use]
pub const fn cookie(previous: u32, count: u32) -> u32 {
    previous ^ ((previous ^ count) & COOKIE_MASK)
}

/// The constants of one cookie-seal routine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CookieSeal {
    /// The word written into the vtable word before the cookie is set.
    pub marker: u32,
    /// The vtable word the routine leaves in the object.
    pub vtable: u32,
    /// The width of the value store.
    pub value_width: ValueWidth,
}

impl CookieSeal {
    /// A seal with the given constants.
    #[must_use]
    pub const fn new(marker: u32, vtable: u32, value_width: ValueWidth) -> Self {
        Self {
            marker,
            vtable,
            value_width,
        }
    }

    /// Stamps `object` with the cookie, advances `counter`, and stores `value`
    /// into the value field. See the module specification.
    ///
    /// # Panics
    ///
    /// When a field lies outside `object`.
    pub fn seal(&self, object: &mut ObjectBytes, counter: &mut u32, value: u32) {
        let count = *counter;
        let previous = object.word(COOKIE_OFFSET);
        object.write(VTABLE_OFFSET, &self.marker.to_le_bytes());
        object.write(COOKIE_OFFSET, &cookie(previous, count).to_le_bytes());
        *counter = count.wrapping_add(1);
        object.write(VTABLE_OFFSET, &self.vtable.to_le_bytes());
        match self.value_width {
            ValueWidth::Word => object.write(VALUE_OFFSET, &value.to_le_bytes()),
            ValueWidth::Byte => object.write(VALUE_OFFSET, &[value as u8]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{COOKIE_MASK, COOKIE_OFFSET, CookieSeal, VALUE_OFFSET, ValueWidth, cookie};
    use crate::field_store::ObjectBytes;

    #[test]
    fn the_cookie_keeps_the_old_bits_outside_the_mask() {
        assert_eq!(cookie(0xFFFF_C000, 0), 0xFFFF_C000);
        assert_eq!(cookie(0, 0x3FFF), 0x3FFF);
        assert_eq!(cookie(0x1234_5678, 0x1234_5678), 0x1234_5678);
        assert_eq!(cookie(0xFFFF_FFFF, 0), 0xFFFF_C000);
        assert_eq!(COOKIE_MASK, 0x3FFF);
    }

    #[test]
    fn a_word_seal_writes_the_fields_in_order_and_advances_the_counter() {
        let seal = CookieSeal::new(0xAAAA_0001, 0xBBBB_0002, ValueWidth::Word);
        let mut object = ObjectBytes::from_bytes(&[0x11; 16]);
        let mut counter = u32::MAX;
        seal.seal(&mut object, &mut counter, 0xCCCC_0003);
        assert_eq!(counter, 0);
        assert_eq!(object.word(0), 0xBBBB_0002);
        assert_eq!(object.word(COOKIE_OFFSET), cookie(0x1111_1111, u32::MAX));
        assert_eq!(object.word(VALUE_OFFSET), 0xCCCC_0003);
        assert_eq!(object.byte(12), 0x11);
    }

    #[test]
    fn a_byte_seal_stores_only_the_low_byte_of_the_value() {
        let seal = CookieSeal::new(1, 2, ValueWidth::Byte);
        let mut object = ObjectBytes::from_bytes(&[0; 16]);
        let mut counter = 5;
        seal.seal(&mut object, &mut counter, 0x1234_56AB);
        assert_eq!(counter, 6);
        assert_eq!(object.byte(VALUE_OFFSET), 0xAB);
        assert_eq!(object.byte(VALUE_OFFSET + 1), 0);
        assert_eq!(object.word(0), 2);
    }
}
