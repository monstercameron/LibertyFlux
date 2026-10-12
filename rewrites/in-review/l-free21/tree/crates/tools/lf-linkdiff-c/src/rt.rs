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
static SLOTS: [AtomicU32; 48] = [AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0)];

/// The slot behind a file address. The address table is generated from the
/// rewrites' own global operands; an unlisted address is a test bug.
fn slot_for(file_va: u32) -> *mut u32 {
    match file_va {
        0x017ACD24 => SLOTS[0].as_ptr(),
        0x0110EF08 => SLOTS[1].as_ptr(),
        0x0110EF44 => SLOTS[2].as_ptr(),
        0x0110EF4C => SLOTS[3].as_ptr(),
        0x0110F0A0 => SLOTS[4].as_ptr(),
        0x0110F068 => SLOTS[5].as_ptr(),
        0x0110F060 => SLOTS[6].as_ptr(),
        0x0110F148 => SLOTS[7].as_ptr(),
        0x0110F130 => SLOTS[8].as_ptr(),
        0x0110F110 => SLOTS[9].as_ptr(),
        0x0110F118 => SLOTS[10].as_ptr(),
        0x0110F150 => SLOTS[11].as_ptr(),
        0x0110F140 => SLOTS[12].as_ptr(),
        0x0110F128 => SLOTS[13].as_ptr(),
        0x0110F138 => SLOTS[14].as_ptr(),
        0x0110F120 => SLOTS[15].as_ptr(),
        0x0110F108 => SLOTS[16].as_ptr(),
        0x0110F18C => SLOTS[17].as_ptr(),
        0x0110F23C => SLOTS[18].as_ptr(),
        0x0110F1CC => SLOTS[19].as_ptr(),
        0x0110F1C4 => SLOTS[20].as_ptr(),
        0x0110F244 => SLOTS[21].as_ptr(),
        0x0110F234 => SLOTS[22].as_ptr(),
        0x0110F24C => SLOTS[23].as_ptr(),
        0x0110FF3C => SLOTS[24].as_ptr(),
        0x0110FF44 => SLOTS[25].as_ptr(),
        0x0110FF34 => SLOTS[26].as_ptr(),
        0x0110FF4C => SLOTS[27].as_ptr(),
        0x0110FF5C => SLOTS[28].as_ptr(),
        0x0110FF9C => SLOTS[29].as_ptr(),
        0x0110FF54 => SLOTS[30].as_ptr(),
        0x0110FFD4 => SLOTS[31].as_ptr(),
        0x0110FF94 => SLOTS[32].as_ptr(),
        0x01110014 => SLOTS[33].as_ptr(),
        0x0110FFFC => SLOTS[34].as_ptr(),
        0x0111000C => SLOTS[35].as_ptr(),
        0x0111001C => SLOTS[36].as_ptr(),
        0x0110FFF4 => SLOTS[37].as_ptr(),
        0x0110FFE4 => SLOTS[38].as_ptr(),
        0x0110FFDC => SLOTS[39].as_ptr(),
        0x0110FFEC => SLOTS[40].as_ptr(),
        0x01110004 => SLOTS[41].as_ptr(),
        0x017AD16C => SLOTS[42].as_ptr(),
        0x01110044 => SLOTS[43].as_ptr(),
        0x011102FC => SLOTS[44].as_ptr(),
        0x011102E4 => SLOTS[45].as_ptr(),
        0x01110314 => SLOTS[46].as_ptr(),
        0x0111032C => SLOTS[47].as_ptr(),
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
