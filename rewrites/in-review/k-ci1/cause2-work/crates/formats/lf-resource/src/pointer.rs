//! Tagged pointers: the 28-bit-offset plus 4-bit-segment-tag references
//! used for every internal reference in a resource.

/// One of the two segments a pointer can target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Segment {
    /// CPU-side data. Tag `5`: values look like `0x5xxxxxxx`.
    System,
    /// Data-side (graphics) data. Tag `6`: values look like `0x6xxxxxxx`.
    Graphics,
}

impl Segment {
    /// The 4-bit tag stored in the top nibble.
    #[must_use]
    pub const fn tag(self) -> u32 {
        match self {
            Segment::System => 5,
            Segment::Graphics => 6,
        }
    }
}

/// A 32-bit tagged resource pointer: top 4 bits select the [`Segment`],
/// low 28 bits are the byte offset within it. Zero is null.
///
/// Some data-segment references pack small flag bits into the low offset
/// bits; [`Pointer::offset`] returns them untouched and the asset reader
/// masks them according to its own layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pointer(u32);

impl Pointer {
    /// Null pointer.
    pub const NULL: Pointer = Pointer(0);
    /// Mask for the 28-bit offset.
    pub const OFFSET_MASK: u32 = 0x0FFF_FFFF;
    /// Debug-fill word the toolchain leaves in unpopulated pointer slots
    /// (not a valid pointer; [`Pointer::segment`] returns `None` for it).
    pub const DEBUG_FILL: u32 = 0xCDCD_CDCD;

    /// Wrap a raw u32 read from a resource.
    #[must_use]
    pub const fn new(raw: u32) -> Pointer {
        Pointer(raw)
    }

    /// The raw value.
    #[must_use]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// True for null (zero). Only null counts; debug fill is *not* null.
    #[must_use]
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }

    /// Which segment this points into: `Some` for tags 5 and 6,
    /// `None` for anything else.
    ///
    /// Null (tag 0) yields `None`: it targets nothing. Debug fill and any
    /// other tag likewise yield `None`.
    #[must_use]
    pub const fn segment(self) -> Option<Segment> {
        match self.0 >> 28 {
            5 => Some(Segment::System),
            6 => Some(Segment::Graphics),
            _ => None,
        }
    }

    /// Byte offset within the segment (low 28 bits), flag bits included.
    #[must_use]
    pub const fn offset(self) -> usize {
        // 28 bits always fit in usize on any supported target.
        (self.0 & Pointer::OFFSET_MASK) as usize
    }
}

impl std::fmt::Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_null() {
            return write!(f, "null");
        }
        match self.segment() {
            Some(Segment::System) => write!(f, "sys+{:#x}", self.offset()),
            Some(Segment::Graphics) => write!(f, "gfx+{:#x}", self.offset()),
            None => write!(f, "bad({:#010x})", self.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_tags() {
        assert_eq!(Pointer::NULL.segment(), None);
        assert!(Pointer::NULL.is_null());
        assert_eq!(Pointer::new(0x5000_0020).segment(), Some(Segment::System));
        assert_eq!(Pointer::new(0x5000_0020).offset(), 0x20);
        assert_eq!(Pointer::new(0x6000_1234).segment(), Some(Segment::Graphics));
        assert_eq!(Pointer::new(0x6000_1234).offset(), 0x1234);
        // High offsets stay inside 28 bits.
        assert_eq!(Pointer::new(0x5FFF_FFFF).offset(), 0x0FFF_FFFF);
        // Debug fill and other tags are not pointers.
        assert_eq!(Pointer::new(Pointer::DEBUG_FILL).segment(), None);
        assert_eq!(Pointer::new(0x0069_5384).segment(), None); // raw vtable
        assert_eq!(Pointer::new(0xBF80_0000).segment(), None); // float -1.0
    }

    #[test]
    fn display_forms() {
        assert_eq!(Pointer::NULL.to_string(), "null");
        assert_eq!(Pointer::new(0x5000_0020).to_string(), "sys+0x20");
        assert_eq!(Pointer::new(0x6000_0100).to_string(), "gfx+0x100");
        assert_eq!(
            Pointer::new(Pointer::DEBUG_FILL).to_string(),
            "bad(0xcdcdcdcd)"
        );
    }
}
