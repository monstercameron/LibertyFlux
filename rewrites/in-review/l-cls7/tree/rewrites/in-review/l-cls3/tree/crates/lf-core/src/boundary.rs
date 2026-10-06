//! The boundary between 32-bit layouts and lifted (native) data.
//!
//! Verified rewrites work on the original's memory: `u32` addresses,
//! `#[repr(C)]` layouts with [`Ptr32`] fields, globals at fixed addresses.
//! Lifted code works on native Rust data: structs with plain fields,
//! [`Handle`]s instead of pointers, `bool`s and enums instead of flag
//! bytes. This module is the only place the two meet. Specification:
//!
//! - **Layouts are codecs.** A 32-bit layout is a `#[repr(C)]` struct that
//!   implements [`FixedLayout`]: it decodes from and encodes to exactly
//!   [`FixedLayout::SIZE`] little-endian bytes, field by field, with no
//!   `unsafe` and no dependence on the host's pointer width or byte order.
//!   [`assert_fixed_layout!`](crate::assert_fixed_layout) ties `SIZE` to
//!   `size_of` at compile time, next to the usual
//!   [`assert_size!`](crate::assert_size) and
//!   [`assert_offset!`](crate::assert_offset) checks.
//! - **Memory is an image.** [`Image32`] is a 32-bit address space seen as
//!   bytes (a test buffer, a save-state snapshot, or, in the transitional
//!   build, the running original). Layouts are read from and written to an
//!   image at an address; nothing outside this module turns a `u32` into a
//!   Rust reference.
//! - **Native types convert explicitly.** [`FromLayout`] builds a native
//!   value from a layout; [`IntoLayout`] writes a native value *into an
//!   existing* layout, so bytes the native type does not model (padding,
//!   fields owned by code not yet lifted) survive a round trip unchanged.
//!   Conversions that meet pointers take an [`AddressMap`] context, which
//!   binds each 32-bit address to a [`Handle`] and back; an address with
//!   no binding is an error, never a guess.
//! - **Opaque identities stay opaque.** A value the lifted cluster carries
//!   but never interprets (the address of an object owned by a cluster not
//!   yet lifted, a type stamp) is a [`Handle32`]: a typed, non-zero 32-bit
//!   cookie. It becomes a real [`Handle`] when its owner lifts.
//!
//! How lifted code carries global state and reaches callees is specified
//! in the `lf-lift` crate documentation; in short, globals become fields of
//! per-subsystem state structs passed explicitly, and callee slots become
//! methods of per-subsystem traits. Both meet this module only when state
//! is loaded from or stored to an [`Image32`].

use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::num::NonZeroU32;
use std::collections::BTreeMap;

use crate::Ptr32;
use crate::arena::Handle;

/// A typed, opaque, non-zero 32-bit identity.
///
/// Four bytes on every target; `Option<Handle32<T>>` is also four bytes,
/// with `None` standing for the original's zero. Unlike [`Ptr32`], which is
/// an address inside a 32-bit layout, a `Handle32` lives in native structs
/// and is never resolved by the code that holds it: it is compared, copied
/// and handed back across the boundary, nothing else.
#[repr(transparent)]
pub struct Handle32<T: ?Sized> {
    raw: NonZeroU32,
    _marker: PhantomData<fn() -> T>,
}

impl<T: ?Sized> Handle32<T> {
    /// Wraps a raw value; `None` for zero.
    #[must_use]
    pub const fn new(raw: u32) -> Option<Self> {
        match NonZeroU32::new(raw) {
            Some(raw) => Some(Self {
                raw,
                _marker: PhantomData,
            }),
            None => None,
        }
    }

    /// The raw value.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.raw.get()
    }

    /// The raw value of an optional handle, zero for `None` (the form the
    /// original stores).
    #[must_use]
    pub const fn raw_or_zero(handle: Option<Self>) -> u32 {
        match handle {
            Some(h) => h.get(),
            None => 0,
        }
    }
}

impl<T: ?Sized> Clone for Handle32<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for Handle32<T> {}

impl<T: ?Sized> PartialEq for Handle32<T> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<T: ?Sized> Eq for Handle32<T> {}

impl<T: ?Sized> Hash for Handle32<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl<T: ?Sized> fmt::Debug for Handle32<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Handle32({:#010x})", self.raw.get())
    }
}

/// Why a boundary conversion failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundaryError {
    /// A non-null 32-bit address has no handle bound to it.
    UnboundAddress(u32),
    /// A handle has no 32-bit address bound to it (handle bits).
    UnboundHandle(u64),
    /// The address or the handle is already bound to something else.
    AlreadyBound(u32),
    /// An access falls outside the image: start address and length.
    OutsideImage {
        /// First address of the access.
        addr: u32,
        /// Length of the access in bytes.
        len: usize,
    },
    /// A field holds a value the native type cannot represent: the
    /// field's address and the raw value.
    Unrepresentable {
        /// Address of the offending field.
        addr: u32,
        /// Its raw value.
        raw: u32,
    },
}

impl fmt::Display for BoundaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnboundAddress(a) => write!(f, "no handle bound to address {a:#010x}"),
            Self::UnboundHandle(h) => write!(f, "no address bound to handle {h:#x}"),
            Self::AlreadyBound(a) => write!(f, "address {a:#010x} or its handle is already bound"),
            Self::OutsideImage { addr, len } => {
                write!(f, "{len} bytes at {addr:#010x} fall outside the image")
            }
            Self::Unrepresentable { addr, raw } => {
                write!(f, "value {raw:#x} at {addr:#010x} has no native form")
            }
        }
    }
}

impl std::error::Error for BoundaryError {}

/// A fixed 32-bit memory layout with an explicit little-endian codec.
///
/// Implementors are `#[repr(C)]` structs (or primitives) whose in-memory
/// size on the 32-bit original is `SIZE`. `decode` and `encode` work field
/// by field, so the codec is the same on every host.
pub trait FixedLayout: Sized {
    /// Size in bytes on the original.
    const SIZE: usize;

    /// Decodes from the first [`Self::SIZE`] bytes of `bytes`.
    ///
    /// # Panics
    ///
    /// When `bytes` is shorter than [`Self::SIZE`] (a caller bug: images
    /// check bounds before decoding).
    fn decode(bytes: &[u8]) -> Self;

    /// Encodes into the first [`Self::SIZE`] bytes of `out`.
    ///
    /// # Panics
    ///
    /// When `out` is shorter than [`Self::SIZE`].
    fn encode(&self, out: &mut [u8]);
}

/// Asserts at compile time that a [`FixedLayout`]'s `SIZE` equals its Rust
/// size, so the codec and the `#[repr(C)]` declaration cannot drift apart.
///
/// # Example
///
/// ```rust
/// use lf_core::assert_fixed_layout;
/// use lf_core::boundary::FixedLayout;
///
/// #[repr(C)]
/// struct Pair {
///     a: u32,
///     b: u32,
/// }
///
/// impl FixedLayout for Pair {
///     const SIZE: usize = 8;
///     fn decode(bytes: &[u8]) -> Self {
///         Self { a: u32::decode(bytes), b: u32::decode(&bytes[4..]) }
///     }
///     fn encode(&self, out: &mut [u8]) {
///         self.a.encode(out);
///         self.b.encode(&mut out[4..]);
///     }
/// }
///
/// assert_fixed_layout!(Pair);
/// ```
#[macro_export]
macro_rules! assert_fixed_layout {
    ($ty:ty) => {
        const _: () = {
            assert!(<$ty as $crate::boundary::FixedLayout>::SIZE == ::core::mem::size_of::<$ty>());
        };
    };
}

macro_rules! fixed_layout_primitive {
    ($($ty:ty),*) => {$(
        impl FixedLayout for $ty {
            const SIZE: usize = core::mem::size_of::<$ty>();
            fn decode(bytes: &[u8]) -> Self {
                let mut raw = [0u8; core::mem::size_of::<$ty>()];
                raw.copy_from_slice(&bytes[..Self::SIZE]);
                <$ty>::from_le_bytes(raw)
            }
            fn encode(&self, out: &mut [u8]) {
                out[..Self::SIZE].copy_from_slice(&self.to_le_bytes());
            }
        }
    )*};
}

fixed_layout_primitive!(u8, i8, u16, i16, u32, i32, u64, i64);

impl FixedLayout for f32 {
    const SIZE: usize = 4;
    /// Bit-exact: NaN payloads survive decoding.
    fn decode(bytes: &[u8]) -> Self {
        f32::from_bits(u32::decode(bytes))
    }
    fn encode(&self, out: &mut [u8]) {
        self.to_bits().encode(out);
    }
}

impl FixedLayout for f64 {
    const SIZE: usize = 8;
    /// Bit-exact: NaN payloads survive decoding.
    fn decode(bytes: &[u8]) -> Self {
        f64::from_bits(u64::decode(bytes))
    }
    fn encode(&self, out: &mut [u8]) {
        self.to_bits().encode(out);
    }
}

impl<T: ?Sized> FixedLayout for Ptr32<T> {
    const SIZE: usize = 4;
    fn decode(bytes: &[u8]) -> Self {
        Ptr32::new(u32::decode(bytes))
    }
    fn encode(&self, out: &mut [u8]) {
        self.addr().encode(out);
    }
}

impl<const N: usize> FixedLayout for [u8; N] {
    const SIZE: usize = N;
    fn decode(bytes: &[u8]) -> Self {
        let mut raw = [0u8; N];
        raw.copy_from_slice(&bytes[..N]);
        raw
    }
    fn encode(&self, out: &mut [u8]) {
        out[..N].copy_from_slice(self);
    }
}

/// A 32-bit address space seen as bytes.
///
/// Implementations decide what backs it (a buffer, a snapshot, the running
/// original). Every access is bounds-checked and reports
/// [`BoundaryError::OutsideImage`] instead of faulting.
pub trait Image32 {
    /// Copies `out.len()` bytes starting at `addr` into `out`.
    ///
    /// # Errors
    ///
    /// [`BoundaryError::OutsideImage`] when any byte is not backed.
    fn read_bytes(&self, addr: u32, out: &mut [u8]) -> Result<(), BoundaryError>;

    /// Copies `bytes` into the image starting at `addr`.
    ///
    /// # Errors
    ///
    /// [`BoundaryError::OutsideImage`] when any byte is not backed.
    fn write_bytes(&mut self, addr: u32, bytes: &[u8]) -> Result<(), BoundaryError>;

    /// Decodes one layout at `addr`.
    ///
    /// # Errors
    ///
    /// As [`Self::read_bytes`].
    fn read<L: FixedLayout>(&self, addr: u32) -> Result<L, BoundaryError> {
        let mut raw = vec![0u8; L::SIZE];
        self.read_bytes(addr, &mut raw)?;
        Ok(L::decode(&raw))
    }

    /// Encodes one layout at `addr`.
    ///
    /// # Errors
    ///
    /// As [`Self::write_bytes`].
    fn write<L: FixedLayout>(&mut self, addr: u32, layout: &L) -> Result<(), BoundaryError> {
        let mut raw = vec![0u8; L::SIZE];
        layout.encode(&mut raw);
        self.write_bytes(addr, &raw)
    }

    /// Decodes `count` consecutive layouts starting at `addr` (a table),
    /// with one bulk read.
    ///
    /// # Errors
    ///
    /// As [`Self::read_bytes`].
    fn read_array<L: FixedLayout>(&self, addr: u32, count: usize) -> Result<Vec<L>, BoundaryError> {
        let mut raw = vec![0u8; L::SIZE * count];
        self.read_bytes(addr, &mut raw)?;
        Ok(raw.chunks_exact(L::SIZE).map(L::decode).collect())
    }

    /// Encodes consecutive layouts starting at `addr`, with one bulk write.
    ///
    /// # Errors
    ///
    /// As [`Self::write_bytes`].
    fn write_array<L: FixedLayout>(&mut self, addr: u32, items: &[L]) -> Result<(), BoundaryError> {
        let mut raw = vec![0u8; L::SIZE * items.len()];
        for (chunk, item) in raw.chunks_exact_mut(L::SIZE).zip(items) {
            item.encode(chunk);
        }
        self.write_bytes(addr, &raw)
    }
}

/// A plain buffer backing the addresses `base .. base + bytes.len()`.
///
/// The reference [`Image32`]: tests and tools build one, fill it through
/// layouts, and compare it byte for byte.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ByteImage {
    /// Address of `bytes[0]`.
    pub base: u32,
    /// The backed bytes.
    pub bytes: Vec<u8>,
}

impl ByteImage {
    /// A zero-filled image of `len` bytes at `base`.
    #[must_use]
    pub fn zeroed(base: u32, len: usize) -> Self {
        Self {
            base,
            bytes: vec![0; len],
        }
    }

    /// The byte range an access covers, if it is fully backed.
    fn span(&self, addr: u32, len: usize) -> Result<core::ops::Range<usize>, BoundaryError> {
        let outside = BoundaryError::OutsideImage { addr, len };
        let start = addr.checked_sub(self.base).ok_or(outside)? as usize;
        let end = start.checked_add(len).ok_or(outside)?;
        if end > self.bytes.len() {
            return Err(outside);
        }
        Ok(start..end)
    }
}

impl Image32 for ByteImage {
    fn read_bytes(&self, addr: u32, out: &mut [u8]) -> Result<(), BoundaryError> {
        let span = self.span(addr, out.len())?;
        out.copy_from_slice(&self.bytes[span]);
        Ok(())
    }

    fn write_bytes(&mut self, addr: u32, bytes: &[u8]) -> Result<(), BoundaryError> {
        let span = self.span(addr, bytes.len())?;
        self.bytes[span].copy_from_slice(bytes);
        Ok(())
    }
}

/// A two-way binding between 32-bit addresses and handles of one type.
///
/// Built when a graph of objects crosses the boundary: each object's
/// address is bound to the handle of its native copy. Null converts to
/// `None` both ways; anything unbound is an error.
#[derive(Clone, Debug)]
pub struct AddressMap<T> {
    to_handle: BTreeMap<u32, Handle<T>>,
    to_addr: BTreeMap<Handle<T>, u32>,
}

impl<T> Default for AddressMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AddressMap<T> {
    /// An empty map.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            to_handle: BTreeMap::new(),
            to_addr: BTreeMap::new(),
        }
    }

    /// Binds a non-null address to a handle.
    ///
    /// # Errors
    ///
    /// [`BoundaryError::AlreadyBound`] when either side is already bound,
    /// or the address is null (null always means `None`).
    pub fn bind(&mut self, addr: u32, handle: Handle<T>) -> Result<(), BoundaryError> {
        if addr == 0 || self.to_handle.contains_key(&addr) || self.to_addr.contains_key(&handle) {
            return Err(BoundaryError::AlreadyBound(addr));
        }
        self.to_handle.insert(addr, handle);
        self.to_addr.insert(handle, addr);
        Ok(())
    }

    /// The handle for a layout pointer: `None` for null.
    ///
    /// # Errors
    ///
    /// [`BoundaryError::UnboundAddress`] for a non-null address with no
    /// binding.
    pub fn handle(&self, ptr: Ptr32<T>) -> Result<Option<Handle<T>>, BoundaryError> {
        if ptr.is_null() {
            return Ok(None);
        }
        self.to_handle
            .get(&ptr.addr())
            .copied()
            .map(Some)
            .ok_or(BoundaryError::UnboundAddress(ptr.addr()))
    }

    /// The layout pointer for an optional handle: null for `None`.
    ///
    /// # Errors
    ///
    /// [`BoundaryError::UnboundHandle`] for a handle with no binding.
    pub fn ptr(&self, handle: Option<Handle<T>>) -> Result<Ptr32<T>, BoundaryError> {
        match handle {
            None => Ok(Ptr32::null()),
            Some(h) => self
                .to_addr
                .get(&h)
                .map(|a| Ptr32::new(*a))
                .ok_or(BoundaryError::UnboundHandle(h.to_bits())),
        }
    }

    /// The address bound to a handle, if any.
    #[must_use]
    pub fn addr_of(&self, handle: Handle<T>) -> Option<u32> {
        self.to_addr.get(&handle).copied()
    }

    /// Number of bindings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.to_handle.len()
    }

    /// True when nothing is bound.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.to_handle.is_empty()
    }
}

/// Builds a native value from its 32-bit layout.
pub trait FromLayout<L>: Sized {
    /// What the conversion needs besides the layout: `()` for plain data,
    /// an [`AddressMap`] (or a struct of them) when pointers are involved.
    type Context: ?Sized;

    /// Converts `layout`.
    ///
    /// # Errors
    ///
    /// When a field has no native form (an unbound pointer, an
    /// out-of-range enum value).
    fn from_layout(layout: &L, cx: &Self::Context) -> Result<Self, BoundaryError>;
}

/// Writes a native value into an existing 32-bit layout.
///
/// Only the fields the native type models are written; every other byte
/// of `out` is left as it was. That is what lets lifted state be stored
/// back over memory that code not yet lifted also uses.
pub trait IntoLayout<L> {
    /// As [`FromLayout::Context`].
    type Context: ?Sized;

    /// Writes `self` into `out`.
    ///
    /// # Errors
    ///
    /// When a value cannot be expressed in the layout (a handle with no
    /// bound address).
    fn write_layout(&self, out: &mut L, cx: &Self::Context) -> Result<(), BoundaryError>;
}

/// Address of element `index` in a table of `stride`-byte elements at
/// `base`, wrapping modulo 2^32 like the original's multiply-and-add.
///
/// Lifted code indexes; this is how the boundary turns an index back into
/// the address a verified rewrite returns.
#[must_use]
pub const fn element_addr(base: u32, index: u32, stride: u32) -> u32 {
    base.wrapping_add(index.wrapping_mul(stride))
}

/// Index of the element at `addr` in a table of `stride`-byte elements at
/// `base`, or `None` when `addr` is below `base`, not on an element start,
/// or `stride` is zero.
#[must_use]
pub const fn element_index(base: u32, addr: u32, stride: u32) -> Option<u32> {
    if stride == 0 || addr < base {
        return None;
    }
    let offset = addr - base;
    if !offset.is_multiple_of(stride) {
        return None;
    }
    Some(offset / stride)
}

#[cfg(test)]
mod tests {
    use super::{
        AddressMap, BoundaryError, ByteImage, FixedLayout, FromLayout, Handle32, Image32,
        IntoLayout, element_addr, element_index,
    };
    use crate::arena::{Arena, Handle};
    use crate::{Ptr32, assert_offset, assert_size};

    // Invented fixtures (they mirror nothing from the game): a 32-bit node
    // layout with a pointer, a payload word and bytes the native side does
    // not model, and its native counterpart.
    #[repr(C)]
    #[derive(Debug, PartialEq)]
    struct NodeLayout {
        next: Ptr32<NodeLayout>,
        value: u32,
        flag: u8,
        reserved: [u8; 3],
    }

    assert_size!(NodeLayout, 12);
    assert_offset!(NodeLayout, next, 0);
    assert_offset!(NodeLayout, value, 4);
    assert_offset!(NodeLayout, flag, 8);
    assert_fixed_layout!(NodeLayout);

    impl FixedLayout for NodeLayout {
        const SIZE: usize = 12;
        fn decode(bytes: &[u8]) -> Self {
            Self {
                next: Ptr32::decode(bytes),
                value: u32::decode(&bytes[4..]),
                flag: u8::decode(&bytes[8..]),
                reserved: <[u8; 3]>::decode(&bytes[9..]),
            }
        }
        fn encode(&self, out: &mut [u8]) {
            self.next.encode(out);
            self.value.encode(&mut out[4..]);
            self.flag.encode(&mut out[8..]);
            self.reserved.encode(&mut out[9..]);
        }
    }

    #[derive(Debug, PartialEq)]
    struct Node {
        next: Option<Handle<Node>>,
        value: u32,
        active: bool,
    }

    impl FromLayout<NodeLayout> for Node {
        type Context = AddressMap<Node>;
        fn from_layout(l: &NodeLayout, cx: &AddressMap<Node>) -> Result<Self, BoundaryError> {
            Ok(Self {
                next: cx.handle(Ptr32::new(l.next.addr()))?,
                value: l.value,
                active: l.flag != 0,
            })
        }
    }

    impl IntoLayout<NodeLayout> for Node {
        type Context = AddressMap<Node>;
        fn write_layout(
            &self,
            out: &mut NodeLayout,
            cx: &AddressMap<Node>,
        ) -> Result<(), BoundaryError> {
            out.next = Ptr32::new(cx.ptr(self.next)?.addr());
            out.value = self.value;
            out.flag = u8::from(self.active);
            Ok(())
        }
    }

    assert_size!(Handle32<u8>, 4);
    assert_size!(Option<Handle32<u8>>, 4);

    #[test]
    fn handle32_zero_is_none() {
        assert!(Handle32::<u8>::new(0).is_none());
        let h = Handle32::<u8>::new(0x1234).unwrap();
        assert_eq!(h.get(), 0x1234);
        assert_eq!(Handle32::raw_or_zero(Some(h)), 0x1234);
        assert_eq!(Handle32::<u8>::raw_or_zero(None), 0);
    }

    #[test]
    fn primitive_codecs_are_little_endian_and_bit_exact() {
        let mut raw = [0u8; 8];
        0x1122_3344u32.encode(&mut raw);
        assert_eq!(raw[..4], [0x44, 0x33, 0x22, 0x11]);
        let nan = f32::from_bits(0x7FA0_0001);
        nan.encode(&mut raw);
        assert_eq!(f32::decode(&raw).to_bits(), 0x7FA0_0001);
        (-2i16).encode(&mut raw);
        assert_eq!(i16::decode(&raw), -2);
    }

    #[test]
    fn graph_round_trip_keeps_unmodelled_bytes() {
        // Two nodes at invented addresses, linked first -> second.
        let (a, b) = (0x1000u32, 0x2000u32);
        let mut image = ByteImage::zeroed(0x1000, 0x1010);
        let first = NodeLayout {
            next: Ptr32::new(b),
            value: 7,
            flag: 2,
            reserved: [0xAA, 0xBB, 0xCC],
        };
        let second = NodeLayout {
            next: Ptr32::null(),
            value: 9,
            flag: 0,
            reserved: [1, 2, 3],
        };
        image.write(a, &first).unwrap();
        image.write(b, &second).unwrap();

        // Allocate native slots first, then bind, then convert.
        let mut arena: Arena<Option<Node>> = Arena::new();
        let ha = arena.insert(None).cast::<Node>();
        let hb = arena.insert(None).cast::<Node>();
        let mut map = AddressMap::new();
        map.bind(a, ha).unwrap();
        map.bind(b, hb).unwrap();
        let na = Node::from_layout(&image.read::<NodeLayout>(a).unwrap(), &map).unwrap();
        let mut nb = Node::from_layout(&image.read::<NodeLayout>(b).unwrap(), &map).unwrap();
        assert_eq!(na.next, Some(hb));
        assert!(na.active);
        assert_eq!(nb.next, None);

        // Lifted code changes the second node; store it back.
        nb.value = 10;
        nb.active = true;
        let mut out = image.read::<NodeLayout>(b).unwrap();
        nb.write_layout(&mut out, &map).unwrap();
        image.write(b, &out).unwrap();
        let back = image.read::<NodeLayout>(b).unwrap();
        assert_eq!(back.value, 10);
        assert_eq!(back.flag, 1);
        assert_eq!(back.reserved, [1, 2, 3], "unmodelled bytes survive");
        // The first node's flag value 2 narrows to `true` and would store
        // back as 1: that narrowing is the converter's documented choice.
        assert_eq!(image.read::<NodeLayout>(a).unwrap().flag, 2);
    }

    #[test]
    fn unbound_and_duplicate_bindings_are_errors() {
        let mut arena = Arena::new();
        let h = arena.insert(());
        let other = arena.insert(());
        let mut map: AddressMap<()> = AddressMap::new();
        assert_eq!(
            map.handle(Ptr32::new(0x40)),
            Err(BoundaryError::UnboundAddress(0x40))
        );
        assert_eq!(map.handle(Ptr32::null()), Ok(None));
        assert_eq!(
            map.ptr(Some(h)),
            Err(BoundaryError::UnboundHandle(h.to_bits()))
        );
        map.bind(0x40, h).unwrap();
        assert_eq!(
            map.bind(0x40, other),
            Err(BoundaryError::AlreadyBound(0x40))
        );
        assert_eq!(map.bind(0x80, h), Err(BoundaryError::AlreadyBound(0x80)));
        assert_eq!(map.bind(0, other), Err(BoundaryError::AlreadyBound(0)));
        assert_eq!(map.ptr(Some(h)).unwrap().addr(), 0x40);
        assert_eq!(map.addr_of(h), Some(0x40));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn image_bounds_are_checked() {
        let mut image = ByteImage::zeroed(0x100, 8);
        assert!(image.write(0x104, &1u32).is_ok());
        assert_eq!(
            image.write(0x106, &1u32),
            Err(BoundaryError::OutsideImage {
                addr: 0x106,
                len: 4
            })
        );
        assert_eq!(
            image.read::<u8>(0xFF),
            Err(BoundaryError::OutsideImage { addr: 0xFF, len: 1 })
        );
        assert_eq!(
            image.read::<u32>(u32::MAX),
            Err(BoundaryError::OutsideImage {
                addr: u32::MAX,
                len: 4
            })
        );
        assert_eq!(image.read::<u32>(0x104), Ok(1));
        image.write_array(0x100, &[0x0201u16, 0x0403]).unwrap();
        assert_eq!(image.read_array::<u8>(0x100, 4).unwrap(), [1, 2, 3, 4]);
        assert!(image.read_array::<u32>(0x100, 3).is_err());
    }

    #[test]
    fn element_address_and_index_are_inverse() {
        assert_eq!(element_addr(0x1000, 3, 0xC8), 0x1000 + 3 * 0xC8);
        assert_eq!(element_addr(0xFFFF_FF00, 2, 0x100), 0x100, "wraps");
        assert_eq!(element_index(0x1000, 0x1000 + 3 * 0xC8, 0xC8), Some(3));
        assert_eq!(element_index(0x1000, 0x1001, 0xC8), None);
        assert_eq!(element_index(0x1000, 0xFFF, 0xC8), None);
        assert_eq!(element_index(0x1000, 0x1000, 0), None);
    }
}
