//! A 32-bit typed pointer for fixed memory layouts.
//!
//! [`Ptr32`] holds a 32-bit address and is 4 bytes on every platform,
//! including 64-bit builds. Layout code uses it anywhere the original used
//! a pointer, so fixed structures keep identical offsets in the 32-bit
//! comparison build and the 64-bit game. It never dereferences memory on
//! its own; conversion to and from real references happens in explicitly
//! marked code at the layout boundary.

use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;

/// A 32-bit address tagged with the pointee type.
///
/// `T` may be unsized; the wrapper itself is always 4 bytes.
#[repr(transparent)]
pub struct Ptr32<T: ?Sized> {
    addr: u32,
    _marker: PhantomData<T>,
}

impl<T: ?Sized> Ptr32<T> {
    /// The null pointer (address zero).
    #[must_use]
    pub const fn null() -> Self {
        Self {
            addr: 0,
            _marker: PhantomData,
        }
    }

    /// Wraps a raw 32-bit address without inspecting it.
    #[must_use]
    pub const fn new(addr: u32) -> Self {
        Self {
            addr,
            _marker: PhantomData,
        }
    }

    /// Returns the wrapped address.
    #[must_use]
    pub const fn addr(self) -> u32 {
        self.addr
    }

    /// Returns true when the address is zero.
    #[must_use]
    pub const fn is_null(self) -> bool {
        self.addr == 0
    }
}

impl<T: ?Sized> Default for Ptr32<T> {
    fn default() -> Self {
        Self::null()
    }
}

impl<T: ?Sized> Clone for Ptr32<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for Ptr32<T> {}

impl<T: ?Sized> PartialEq for Ptr32<T> {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}

impl<T: ?Sized> Eq for Ptr32<T> {}

impl<T: ?Sized> Hash for Ptr32<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.addr.hash(state);
    }
}

impl<T: ?Sized> fmt::Debug for Ptr32<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ptr32({:#010x})", self.addr)
    }
}
