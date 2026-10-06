//! Small pool allocators: the bare bump pool and published objects.
//!
//! Lifted from three verified routines (no class): the bump allocator
//! that hands out 32-byte slots from a global count and base without
//! registering them, and the two create-and-publish instances (one
//! routine, proved twice) that allocate a pool object, initialise it
//! with constant parameters and publish it into a global slot.

use lf_core::boundary::Handle32;

/// Shift of the bump pool's 32-byte stride.
pub const BUMP_SHIFT: u32 = 5;
/// Size in bytes of a published pool object.
pub const OBJ_SIZE: u32 = 0x1c;

/// A bare bump pool: the slot count, bumped past every handed-out slot.
/// The element base is not owned (it is a global address in the original)
/// and travels as a call argument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BumpPool {
    /// Slots handed out so far.
    pub count: u32,
}

impl BumpPool {
    /// Hands out slot `count` at `base + count * 32` (wrapping) and bumps
    /// the count past it.
    pub fn next(&mut self, base: u32) -> u32 {
        let slot = self.count.wrapping_shl(BUMP_SHIFT).wrapping_add(base);
        self.count = self.count.wrapping_add(1);
        slot
    }
}

/// Identity of a published pool object.
#[derive(Debug)]
pub struct ObjTag;

/// An opaque published pool object: the creation routines' answer.
pub type ObjHandle = Handle32<ObjTag>;

/// Identity of a pool-object vtable: the creation parameter's meaning.
///
/// Vtables live in code not yet lifted, so they travel as opaque handles.
#[derive(Debug)]
pub struct ObjVtabTag;

/// An opaque pool-object vtable word.
pub type ObjVtable = Handle32<ObjVtabTag>;

/// Allocates a published pool object: the creation routines' allocator.
pub trait ObjAlloc {
    /// Allocates `size` bytes, returning the block or `None` on failure.
    fn alloc(&mut self, size: u32) -> Option<ObjHandle>;
}

/// Initialises a published pool object: the creation routines' initialiser.
pub trait ObjInit {
    /// Initialises `block` with the instance's parameters, returning the
    /// object to publish.
    fn init(&mut self, block: ObjHandle, a: u32, vtable: ObjVtable, c: u32) -> ObjHandle;
}

impl<F: FnMut(u32) -> Option<ObjHandle>> ObjAlloc for F {
    fn alloc(&mut self, size: u32) -> Option<ObjHandle> {
        self(size)
    }
}

impl<F: FnMut(ObjHandle, u32, ObjVtable, u32) -> ObjHandle> ObjInit for F {
    fn init(&mut self, block: ObjHandle, a: u32, vtable: ObjVtable, c: u32) -> ObjHandle {
        self(block, a, vtable, c)
    }
}

/// A published pool object, created and installed globally.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublishedObj;

impl PublishedObj {
    /// Allocates the object; when that fails returns `None` (the original
    /// clears its global slot and returns 0). Otherwise the block is
    /// initialised with the instance's parameters and the initialiser's
    /// answer is returned (the original publishes it into its global slot
    /// and returns it too).
    pub fn create(
        a: u32,
        c: u32,
        vtable: ObjVtable,
        alloc: &mut impl ObjAlloc,
        init: &mut impl ObjInit,
    ) -> Option<ObjHandle> {
        let block = alloc.alloc(OBJ_SIZE)?;
        Some(init.init(block, a, vtable, c))
    }
}
