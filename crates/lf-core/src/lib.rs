//! `lf-core`: shared types for every engine crate.
//!
//! README for future lanes:
//! - Fixed-layout helpers live here: [`Ptr32`] (a 32-bit typed pointer that
//!   stays 4 bytes on every platform) and the [`assert_size!`] /
//!   [`assert_offset!`] compile-time layout assertions.
//! - The lift's boundary types live here too: [`arena`] ([`Handle`] and
//!   [`Arena`], how lifted code refers to objects) and [`boundary`]
//!   ([`Handle32`] cookies, the [`FixedLayout`](boundary::FixedLayout)
//!   codec, [`Image32`](boundary::Image32) address spaces, the
//!   [`AddressMap`](boundary::AddressMap) and the conversion traits between
//!   32-bit layouts and native structs). The method they serve is specified
//!   in the `lf-lift` crate documentation.
//! - The game's maths types will live here once the `glam` dependency is
//!   approved (see the t-engine-crates report); nothing invents its own
//!   vector/matrix types elsewhere.
//! - This crate must stay dependency-free and portable: it builds for every
//!   target including `i686-pc-windows-msvc`, and it must never contain
//!   original game code, only new Rust.

pub mod arena;
pub mod boundary;
pub mod layout_assert;
pub mod ptr32;

pub use arena::{Arena, Handle};
pub use boundary::Handle32;
pub use ptr32::Ptr32;

#[cfg(test)]
mod tests {
    use super::Ptr32;
    use crate::{assert_offset, assert_size};

    // Two invented example structures exercised by the layout assertions.
    // They are test fixtures only and mirror nothing from the game.
    #[repr(C)]
    struct DemoHeader {
        magic: u32,
        version: u16,
        flags: u16,
        payload_len: u32,
    }

    assert_size!(DemoHeader, 12);
    assert_offset!(DemoHeader, magic, 0);
    assert_offset!(DemoHeader, version, 4);
    assert_offset!(DemoHeader, flags, 6);
    assert_offset!(DemoHeader, payload_len, 8);

    #[repr(C)]
    struct DemoNode {
        tag: u16,
        kind: u16,
        next: Ptr32<DemoNode>,
        value: f32,
    }

    assert_size!(DemoNode, 12);
    assert_offset!(DemoNode, tag, 0);
    assert_offset!(DemoNode, next, 4);
    assert_offset!(DemoNode, value, 8);

    // Ptr32 is 4 bytes even on 64-bit hosts: this is what keeps fixed
    // layouts identical between the 32-bit comparison build and the
    // 64-bit game.
    assert_size!(Ptr32<u8>, 4);

    #[test]
    fn demo_layouts_read_back() {
        let header = DemoHeader {
            magic: 0x464C_5857,
            version: 3,
            flags: 1,
            payload_len: 64,
        };
        assert_eq!(header.magic, 0x464C_5857);
        assert_eq!(header.version, 3);
        assert_eq!(header.flags, 1);
        assert_eq!(header.payload_len, 64);

        let node = DemoNode {
            tag: 7,
            kind: 9,
            next: Ptr32::null(),
            value: 1.5,
        };
        assert_eq!(node.tag, 7);
        assert_eq!(node.kind, 9);
        assert!(node.next.is_null());
        assert!((node.value - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn null_pointer_compares_equal_to_zero_address() {
        assert_eq!(Ptr32::<u8>::null(), Ptr32::new(0));
        assert!(!Ptr32::<u8>::new(16).is_null());
        assert_eq!(Ptr32::<u8>::new(16).addr(), 16);
    }
}
