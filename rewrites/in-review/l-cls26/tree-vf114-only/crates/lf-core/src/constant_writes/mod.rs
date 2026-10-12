//! Constant writes: a fixed list of fields of one object set to constants, and a fixed return.
//!
//! Several small routines of the original do nothing but initialise an object: they write constants into a
//! few fields and return zero, one, or the object itself. Each write has a width (one byte, two bytes or four
//! bytes), a field offset and a value. [`ConstantWrite`] describes one write as data; [`ConstantWrites`]
//! describes one routine. [`ConstantWrites::apply`] is the routine, and [`apply_writes`] is the same behaviour
//! for a list the caller builds. The instances the verified rewrites were checked against are listed in
//! [`instances`].
//!
//! Specification of [`apply_writes`]:
//!
//! - The writes happen in the order of the list. Each writes its value, truncated to its width, least
//!   significant byte first, at its offset, and nothing else.
//! - The result is `0` for [`Returns::Zero`], `1` for [`Returns::One`], and the object's own address (the
//!   value passed as `this`) for [`Returns::This`].
//! - A write that does not lie inside the object is a caller error and panics.
//! - Nothing here loops beyond the list or allocates.

use crate::field_store::ObjectBytes;

pub mod instances;

/// The width of one constant write.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteWidth {
    /// One byte.
    Byte,
    /// Two bytes.
    Half,
    /// Four bytes.
    Word,
}

/// One write of a constant into a field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstantWrite {
    /// Byte offset of the field from the start of the object.
    pub offset: usize,
    /// Width of the field.
    pub width: WriteWidth,
    /// The constant, truncated to the width when written.
    pub value: u32,
}

/// What the routine returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Returns {
    /// Zero.
    Zero,
    /// One.
    One,
    /// The object's own address, which the caller passes in.
    This,
}

/// One routine of this family: its writes and its return.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstantWrites {
    /// The writes, in order.
    pub writes: &'static [ConstantWrite],
    /// What the routine returns.
    pub returns: Returns,
}

impl ConstantWrites {
    /// Runs the routine on `object`, whose own address is `this`, and returns its result.
    ///
    /// # Panics
    ///
    /// When a write does not lie inside the object.
    pub fn apply(&self, object: &mut ObjectBytes, this: u32) -> u32 {
        apply_writes(self.writes, self.returns, object, this)
    }
}

/// Runs a list of writes and a return on `object`, whose own address is `this`. See the module
/// specification.
///
/// # Panics
///
/// When a write does not lie inside the object.
pub fn apply_writes(
    writes: &[ConstantWrite],
    returns: Returns,
    object: &mut ObjectBytes,
    this: u32,
) -> u32 {
    for write in writes {
        match write.width {
            WriteWidth::Byte => object.write(write.offset, &[write.value as u8]),
            WriteWidth::Half => object.write(write.offset, &(write.value as u16).to_le_bytes()),
            WriteWidth::Word => object.write(write.offset, &write.value.to_le_bytes()),
        }
    }
    match returns {
        Returns::Zero => 0,
        Returns::One => 1,
        Returns::This => this,
    }
}

#[cfg(test)]
mod tests {
    use super::{ConstantWrite, Returns, WriteWidth, apply_writes};
    use crate::field_store::ObjectBytes;

    #[test]
    fn each_width_writes_its_own_number_of_bytes() {
        let writes = [
            ConstantWrite {
                offset: 0,
                width: WriteWidth::Byte,
                value: 0x1FF,
            },
            ConstantWrite {
                offset: 2,
                width: WriteWidth::Half,
                value: 0xABCD_1234,
            },
            ConstantWrite {
                offset: 6,
                width: WriteWidth::Word,
                value: 0x0102_0304,
            },
        ];
        let mut object = ObjectBytes::zeroed(12);
        let result = apply_writes(&writes, Returns::One, &mut object, 77);
        assert_eq!(result, 1);
        assert_eq!(
            object.bytes(),
            &[0xFF, 0, 0x34, 0x12, 0, 0, 4, 3, 2, 1, 0, 0]
        );
    }

    #[test]
    fn this_returns_the_address_passed_in_and_zero_returns_zero() {
        let mut object = ObjectBytes::zeroed(4);
        assert_eq!(
            apply_writes(&[], Returns::This, &mut object, 0x1234),
            0x1234
        );
        assert_eq!(apply_writes(&[], Returns::Zero, &mut object, 0x1234), 0);
    }

    #[test]
    #[should_panic(expected = "lies outside an object")]
    fn a_write_past_the_end_panics() {
        let writes = [ConstantWrite {
            offset: 3,
            width: WriteWidth::Word,
            value: 0,
        }];
        let mut object = ObjectBytes::zeroed(4);
        let _ = apply_writes(&writes, Returns::Zero, &mut object, 0);
    }
}
