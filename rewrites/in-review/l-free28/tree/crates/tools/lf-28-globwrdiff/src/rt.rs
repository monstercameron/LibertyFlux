//! The 32-bit differential runtime for the global constant writes of this crate.
//!
//! The rewrites write absolute global addresses. Here one byte region stands for them: the address `base` the test
//! sets is the region's first byte, and every global the rewrites touch lies inside it.

// Test-only runtime: raw pointers into the region are inherent here. Every access is bounds-checked against the
// region before it is made, and the region is only touched by the one test lock.
#![allow(unsafe_code)]

/// Bytes in the region. Every global an instance of this crate touches lies inside it.
pub const REGION_BYTES: usize = 16384;

/// The region. One test program at a time uses it, under the test lock.
static mut REGION: [u8; REGION_BYTES] = [0; REGION_BYTES];

/// The global address of the region's first byte.
static mut REGION_BASE: u32 = 0;

/// Points the globals at a region that starts at the global address `base`.
pub fn set_region(base: u32) {
    // SAFETY: the test lock serialises the programs that use the region, so nothing else reads or writes it.
    unsafe {
        REGION_BASE = base;
    }
}

/// Offset of a global address in the region. An address outside the region is a test bug.
fn offset_for(file_va: u32) -> usize {
    // SAFETY: as above.
    let base = unsafe { REGION_BASE };
    let offset = file_va.wrapping_sub(base) as usize;
    assert!(offset < REGION_BYTES, "global VA {file_va:#x} lies outside the region");
    offset
}

/// Pointer to the global at a global address, mirroring `lf-checker-rt::global`.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    let offset = offset_for(file_va);
    // SAFETY: the offset is inside the region, and the region is one static array.
    unsafe { core::ptr::addr_of_mut!(REGION).cast::<u8>().add(offset).cast::<T>() }
}

/// The address of the global, as the rewrites compute with it, mirroring `lf-checker-rt::relocated`.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    global::<u8>(file_va) as usize as u32
}

/// Writes the whole region from `bytes`.
pub fn fill_region(bytes: &[u8]) {
    // SAFETY: as above; the copy stays inside the region.
    unsafe {
        core::ptr::addr_of_mut!(REGION).cast::<u8>().copy_from(bytes.as_ptr(), REGION_BYTES);
    }
}

/// The region's bytes as they are now.
#[must_use]
pub fn region_bytes() -> Vec<u8> {
    // SAFETY: as above.
    unsafe { core::ptr::addr_of!(REGION).cast::<u8>().cast::<[u8; REGION_BYTES]>().read().to_vec() }
}

/// Declare a rewrite export with the original's calling convention.
/// Mirrors `lf-checker-rt::export`.
#[macro_export]
macro_rules! export {
    (cdecl, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (cdecl).
        pub extern "cdecl" fn $name($($arg : $ty),*) -> $ret $body
    };
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
}
