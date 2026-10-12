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
static SLOTS: [AtomicU32; 60] = [
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
];

/// The slot behind a file address. The address table is generated from the
/// rewrites' own global operands; an unlisted address is a test bug.
fn slot_for(file_va: u32) -> *mut u32 {
    match file_va {
        0x017AD1B8 => SLOTS[0].as_ptr(),
        0x0110EA04 => SLOTS[1].as_ptr(),
        0x0110E994 => SLOTS[2].as_ptr(),
        0x0110E984 => SLOTS[3].as_ptr(),
        0x0110EA8C => SLOTS[4].as_ptr(),
        0x0110EA6C => SLOTS[5].as_ptr(),
        0x0110EA7C => SLOTS[6].as_ptr(),
        0x0110EA5C => SLOTS[7].as_ptr(),
        0x0110EAAC => SLOTS[8].as_ptr(),
        0x0110EA9C => SLOTS[9].as_ptr(),
        0x017ACD24 => SLOTS[10].as_ptr(),
        0x0110EAFC => SLOTS[11].as_ptr(),
        0x0110EB04 => SLOTS[12].as_ptr(),
        0x0110EAE4 => SLOTS[13].as_ptr(),
        0x0110EB2C => SLOTS[14].as_ptr(),
        0x0110EBC4 => SLOTS[15].as_ptr(),
        0x0110EB3C => SLOTS[16].as_ptr(),
        0x0110EB64 => SLOTS[17].as_ptr(),
        0x0110EB74 => SLOTS[18].as_ptr(),
        0x0110EB84 => SLOTS[19].as_ptr(),
        0x0110EBA4 => SLOTS[20].as_ptr(),
        0x0110EB4C => SLOTS[21].as_ptr(),
        0x0110EBE4 => SLOTS[22].as_ptr(),
        0x0110EBD4 => SLOTS[23].as_ptr(),
        0x0110EBB4 => SLOTS[24].as_ptr(),
        0x0110EB94 => SLOTS[25].as_ptr(),
        0x0110EB0C => SLOTS[26].as_ptr(),
        0x0110EB54 => SLOTS[27].as_ptr(),
        0x0110EC0C => SLOTS[28].as_ptr(),
        0x0110EC6C => SLOTS[29].as_ptr(),
        0x0110EC34 => SLOTS[30].as_ptr(),
        0x0110EC7C => SLOTS[31].as_ptr(),
        0x0110EC1C => SLOTS[32].as_ptr(),
        0x0110EC9C => SLOTS[33].as_ptr(),
        0x0110EC44 => SLOTS[34].as_ptr(),
        0x0110EC54 => SLOTS[35].as_ptr(),
        0x0110EC04 => SLOTS[36].as_ptr(),
        0x0110ECBC => SLOTS[37].as_ptr(),
        0x0110EC64 => SLOTS[38].as_ptr(),
        0x0110EC8C => SLOTS[39].as_ptr(),
        0x0110EBF4 => SLOTS[40].as_ptr(),
        0x0110EC2C => SLOTS[41].as_ptr(),
        0x0110ECAC => SLOTS[42].as_ptr(),
        0x0110ECC4 => SLOTS[43].as_ptr(),
        0x0110ECFC => SLOTS[44].as_ptr(),
        0x0110EE0C => SLOTS[45].as_ptr(),
        0x0110EE04 => SLOTS[46].as_ptr(),
        0x0110EE28 => SLOTS[47].as_ptr(),
        0x0110EE30 => SLOTS[48].as_ptr(),
        0x0110EE38 => SLOTS[49].as_ptr(),
        0x0110EE80 => SLOTS[50].as_ptr(),
        0x0110EE78 => SLOTS[51].as_ptr(),
        0x0110EE70 => SLOTS[52].as_ptr(),
        0x0110EE90 => SLOTS[53].as_ptr(),
        0x0110EE88 => SLOTS[54].as_ptr(),
        0x0110EE98 => SLOTS[55].as_ptr(),
        0x0110EEA8 => SLOTS[56].as_ptr(),
        0x0110EEA0 => SLOTS[57].as_ptr(),
        0x0110EEEC => SLOTS[58].as_ptr(),
        0x0110EF00 => SLOTS[59].as_ptr(),
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
