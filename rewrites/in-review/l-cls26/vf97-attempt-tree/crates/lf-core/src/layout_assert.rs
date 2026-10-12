//! Compile-time layout assertions for fixed 32-bit memory layouts.
//!
//! Every structure that mirrors the original's memory must carry an
//! [`assert_size!`] check, and every field a caller depends on carries an
//! [`assert_offset!`] check. Failures are compile errors, so a layout that
//! drifts on any target breaks the build instead of corrupting memory.

/// Asserts at compile time that a type has an exact size in bytes.
///
/// Put one on every `#[repr(C)]` type in a subsystem `layout` module.
///
/// # Example
///
/// ```rust
/// use lf_core::assert_size;
///
/// #[repr(C)]
/// struct Pair {
///     a: u32,
///     b: u32,
/// }
///
/// assert_size!(Pair, 8);
/// ```
#[macro_export]
macro_rules! assert_size {
    ($ty:ty, $size:expr) => {
        const _: () = {
            assert!(::core::mem::size_of::<$ty>() == $size);
        };
    };
}

/// Asserts at compile time that a field sits at an exact byte offset.
///
/// Only named fields are supported (`offset_of!` paths through tuple
/// fields are not). Nested paths such as `outer.inner` are allowed.
///
/// # Example
///
/// ```rust
/// use lf_core::assert_offset;
///
/// #[repr(C)]
/// struct Pair {
///     a: u32,
///     b: u16,
///     c: u16,
/// }
///
/// assert_offset!(Pair, a, 0);
/// assert_offset!(Pair, b, 4);
/// assert_offset!(Pair, c, 6);
/// ```
#[macro_export]
macro_rules! assert_offset {
    ($ty:ty, $($field:ident).+, $offset:expr) => {
        const _: () = {
            assert!(::core::mem::offset_of!($ty, $($field).+) == $offset);
        };
    };
}
