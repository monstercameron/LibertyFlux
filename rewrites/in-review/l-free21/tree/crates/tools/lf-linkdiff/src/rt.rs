//! The 32-bit differential runtime: the checker's global and relocation names.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, `global`, `relocated`). Each global the rewrites in this crate
//! touch is one stable word; the address-to-slot table below lists them and
//! nothing else. A relocated address is the file address itself in this
//! runtime, which is all the comparison needs: the lifted side is given the
//! same value through the instance table.

// Test-only runtime: raw addresses through scripted globals are inherent
// here. Every access stays inside the test memory the case planted.
#![allow(unsafe_code)]

use core::sync::atomic::AtomicU32;

/// One stable word per global the rewrites of this crate touch.
static SLOTS: [AtomicU32; 61] = [AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0)];

/// The slot behind a file address. The address table is generated from the
/// rewrites' own global operands; an unlisted address is a test bug.
fn slot_for(file_va: u32) -> *mut u32 {
    match file_va {
        0x017AD16C => SLOTS[0].as_ptr(),
        0x0110DBDC => SLOTS[1].as_ptr(),
        0x0110DBC4 => SLOTS[2].as_ptr(),
        0x0110DBAC => SLOTS[3].as_ptr(),
        0x017AD1B8 => SLOTS[4].as_ptr(),
        0x0110DC00 => SLOTS[5].as_ptr(),
        0x0110DC20 => SLOTS[6].as_ptr(),
        0x0110DC10 => SLOTS[7].as_ptr(),
        0x0110DBF0 => SLOTS[8].as_ptr(),
        0x0110DD84 => SLOTS[9].as_ptr(),
        0x0110DD94 => SLOTS[10].as_ptr(),
        0x0110DD40 => SLOTS[11].as_ptr(),
        0x0110DDC0 => SLOTS[12].as_ptr(),
        0x0110DC64 => SLOTS[13].as_ptr(),
        0x0110DC34 => SLOTS[14].as_ptr(),
        0x0110DD28 => SLOTS[15].as_ptr(),
        0x0110DCF4 => SLOTS[16].as_ptr(),
        0x0110DC7C => SLOTS[17].as_ptr(),
        0x0110DD58 => SLOTS[18].as_ptr(),
        0x0110DC4C => SLOTS[19].as_ptr(),
        0x0110DC94 => SLOTS[20].as_ptr(),
        0x0110DD0C => SLOTS[21].as_ptr(),
        0x0110DD70 => SLOTS[22].as_ptr(),
        0x0110DDA8 => SLOTS[23].as_ptr(),
        0x0110DCC4 => SLOTS[24].as_ptr(),
        0x0110DCAC => SLOTS[25].as_ptr(),
        0x0110DCDC => SLOTS[26].as_ptr(),
        0x0110DDF0 => SLOTS[27].as_ptr(),
        0x0110DE50 => SLOTS[28].as_ptr(),
        0x0110DE20 => SLOTS[29].as_ptr(),
        0x0110DE68 => SLOTS[30].as_ptr(),
        0x0110DE08 => SLOTS[31].as_ptr(),
        0x0110DE80 => SLOTS[32].as_ptr(),
        0x0110DE98 => SLOTS[33].as_ptr(),
        0x0110DDD8 => SLOTS[34].as_ptr(),
        0x0110DE38 => SLOTS[35].as_ptr(),
        0x0110DEDC => SLOTS[36].as_ptr(),
        0x0110DEFC => SLOTS[37].as_ptr(),
        0x0110DEBC => SLOTS[38].as_ptr(),
        0x0110DECC => SLOTS[39].as_ptr(),
        0x0110DEEC => SLOTS[40].as_ptr(),
        0x0110DEAC => SLOTS[41].as_ptr(),
        0x017ACD24 => SLOTS[42].as_ptr(),
        0x0110E138 => SLOTS[43].as_ptr(),
        0x0110E178 => SLOTS[44].as_ptr(),
        0x0110E170 => SLOTS[45].as_ptr(),
        0x0110E6A8 => SLOTS[46].as_ptr(),
        0x0110E6C4 => SLOTS[47].as_ptr(),
        0x0110E6F4 => SLOTS[48].as_ptr(),
        0x0110EA1C => SLOTS[49].as_ptr(),
        0x0110E9E4 => SLOTS[50].as_ptr(),
        0x0110E9D4 => SLOTS[51].as_ptr(),
        0x0110E9C4 => SLOTS[52].as_ptr(),
        0x0110E9A4 => SLOTS[53].as_ptr(),
        0x0110E974 => SLOTS[54].as_ptr(),
        0x0110EA2C => SLOTS[55].as_ptr(),
        0x0110E9F4 => SLOTS[56].as_ptr(),
        0x0110EA3C => SLOTS[57].as_ptr(),
        0x0110EA14 => SLOTS[58].as_ptr(),
        0x0110EA4C => SLOTS[59].as_ptr(),
        0x0110E9B4 => SLOTS[60].as_ptr(),
        _ => panic!("unexpected global VA {file_va:#x}"),
    }
}

/// Pointer to the global at a file VA, mirroring `lf-checker-rt::global`.
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    slot_for(file_va) as *mut T
}

/// The relocated address of a file VA. This runtime maps a file VA to
/// itself, so the rewrites' relocated operands and the instance table agree.
#[must_use]
pub fn relocated(file_va: u32) -> u32 {
    file_va
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
