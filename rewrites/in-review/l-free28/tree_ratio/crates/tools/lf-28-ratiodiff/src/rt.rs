//! The 32-bit differential runtime: the checker macro and global slots.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, `global`). Each global the rewrites in this crate touch is one
//! stable word; the address-to-slot table below lists them and nothing else.
//! The tests hold one lock across each whole test, so the rewrite's plain
//! reads and writes through the slots never race.

// Test-only runtime: raw addresses through scripted globals are inherent
// here. Every access stays inside the test memory the case planted.
#![allow(unsafe_code)]

use core::sync::atomic::AtomicU32;

/// One stable word per global the rewrites of this crate touch.
static SLOTS: [AtomicU32; 30] = [AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0)];

/// The slot behind a file address. The address table is generated from the
/// rewrites' own global operands; an unlisted address is a test bug.
fn slot_for(file_va: u32) -> *mut u32 {
    match file_va {
        0x01048274 => SLOTS[0].as_ptr(),
        0x01048278 => SLOTS[1].as_ptr(),
        0x016B9D74 => SLOTS[2].as_ptr(),
        0x01048340 => SLOTS[3].as_ptr(),
        0x01048344 => SLOTS[4].as_ptr(),
        0x016C8E04 => SLOTS[5].as_ptr(),
        0x01048400 => SLOTS[6].as_ptr(),
        0x01048404 => SLOTS[7].as_ptr(),
        0x016C8F90 => SLOTS[8].as_ptr(),
        0x0104842C => SLOTS[9].as_ptr(),
        0x01048430 => SLOTS[10].as_ptr(),
        0x016CF3C0 => SLOTS[11].as_ptr(),
        0x01048448 => SLOTS[12].as_ptr(),
        0x0104844C => SLOTS[13].as_ptr(),
        0x016CF3C4 => SLOTS[14].as_ptr(),
        0x01048464 => SLOTS[15].as_ptr(),
        0x01048468 => SLOTS[16].as_ptr(),
        0x016CFD74 => SLOTS[17].as_ptr(),
        0x010484D0 => SLOTS[18].as_ptr(),
        0x010484D4 => SLOTS[19].as_ptr(),
        0x016CFD78 => SLOTS[20].as_ptr(),
        0x01048500 => SLOTS[21].as_ptr(),
        0x01048504 => SLOTS[22].as_ptr(),
        0x016D00E8 => SLOTS[23].as_ptr(),
        0x01048A70 => SLOTS[24].as_ptr(),
        0x01048A74 => SLOTS[25].as_ptr(),
        0x016D11F8 => SLOTS[26].as_ptr(),
        0x01048AAC => SLOTS[27].as_ptr(),
        0x01048AB0 => SLOTS[28].as_ptr(),
        0x016D2110 => SLOTS[29].as_ptr(),
        _ => panic!("unexpected global VA {file_va:#x}"),
    }
}

/// Pointer to the global at a file VA, mirroring `lf-checker-rt::global`.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    slot_for(file_va) as *mut T
}

/// Plants `bits` at the global `file_va`.
pub fn plant(file_va: u32, bits: u32) {
    unsafe {
        global::<u32>(file_va).write(bits);
    }
}

/// Reads back the word at `file_va` through a fresh raw pointer, so the
/// compiler cannot forward a value the rewrite overwrote.
#[must_use]
pub fn read_back(file_va: u32) -> u32 {
    unsafe { (global::<u32>(file_va) as *const u32).read() }
}

/// Declare a rewrite export with the original's calling convention.
/// Mirrors `lf-checker-rt::export`.
#[macro_export]
macro_rules! export {
    (cdecl, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (cdecl).
        #[unsafe(no_mangle)]
        pub extern "cdecl" fn $name($($arg : $ty),*) -> $ret $body
    };
}
