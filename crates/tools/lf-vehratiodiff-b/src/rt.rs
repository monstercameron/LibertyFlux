//! The 32-bit differential runtime: the checker macro and global slots.
//!
//! Mirrors the surface the checker builds verified rewrites against
//! (`export!`, `global`). Every instance in this crate touches exactly
//! three globals (numerator, denominator, quotient), each served by one
//! atomic slot; the atomics give the slots stable addresses, and the
//! differential tests hold one lock across each whole test, so the
//! rewrite's plain reads and writes through them never race. Test-support
//! code: the lifted crate itself stays `#![forbid(unsafe_code)]`.

// Test-only runtime: raw addresses through scripted globals are inherent
// here. Every access stays inside the test memory the case planted.
#![allow(unsafe_code)]

use core::sync::atomic::AtomicU32;

/// One stable word per global the proof set touches.
static SLOTS: [AtomicU32; 168] = [
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

/// Address of the global slot for a file VA.
///
/// # Panics
///
/// When the address is not one of the globals the proof set touches:
/// a case bug, never a guess.
fn slot_for(file_va: u32) -> *mut u32 {
    match file_va {
        0x0105_0D50 => SLOTS[0].as_ptr(),
        0x0105_0D54 => SLOTS[1].as_ptr(),
        0x0171_BBC4 => SLOTS[2].as_ptr(),
        0x0105_0D70 => SLOTS[3].as_ptr(),
        0x0105_0D74 => SLOTS[4].as_ptr(),
        0x0171_BBC8 => SLOTS[5].as_ptr(),
        0x0105_0DF4 => SLOTS[6].as_ptr(),
        0x0105_0DF8 => SLOTS[7].as_ptr(),
        0x0171_BBCC => SLOTS[8].as_ptr(),
        0x0105_0E20 => SLOTS[9].as_ptr(),
        0x0105_0E24 => SLOTS[10].as_ptr(),
        0x0171_BC0C => SLOTS[11].as_ptr(),
        0x0105_1414 => SLOTS[12].as_ptr(),
        0x0105_1418 => SLOTS[13].as_ptr(),
        0x0171_C0D4 => SLOTS[14].as_ptr(),
        0x0105_1590 => SLOTS[15].as_ptr(),
        0x0105_1594 => SLOTS[16].as_ptr(),
        0x0171_C10C => SLOTS[17].as_ptr(),
        0x0105_15E4 => SLOTS[18].as_ptr(),
        0x0105_15E8 => SLOTS[19].as_ptr(),
        0x0171_C12C => SLOTS[20].as_ptr(),
        0x0105_1690 => SLOTS[21].as_ptr(),
        0x0105_1694 => SLOTS[22].as_ptr(),
        0x0171_C934 => SLOTS[23].as_ptr(),
        0x0105_1964 => SLOTS[24].as_ptr(),
        0x0105_1968 => SLOTS[25].as_ptr(),
        0x0171_CA14 => SLOTS[26].as_ptr(),
        0x0105_1A3C => SLOTS[27].as_ptr(),
        0x0105_1A40 => SLOTS[28].as_ptr(),
        0x0171_CA2C => SLOTS[29].as_ptr(),
        0x0105_1AE0 => SLOTS[30].as_ptr(),
        0x0105_1AE4 => SLOTS[31].as_ptr(),
        0x0171_CA30 => SLOTS[32].as_ptr(),
        0x0105_21C4 => SLOTS[33].as_ptr(),
        0x0105_21C8 => SLOTS[34].as_ptr(),
        0x0171_D020 => SLOTS[35].as_ptr(),
        0x0105_24B4 => SLOTS[36].as_ptr(),
        0x0105_24B8 => SLOTS[37].as_ptr(),
        0x0171_D77C => SLOTS[38].as_ptr(),
        0x0105_34B0 => SLOTS[39].as_ptr(),
        0x0105_34B4 => SLOTS[40].as_ptr(),
        0x0171_DAD0 => SLOTS[41].as_ptr(),
        0x0105_3910 => SLOTS[42].as_ptr(),
        0x0105_3914 => SLOTS[43].as_ptr(),
        0x0171_DAE8 => SLOTS[44].as_ptr(),
        0x0105_3918 => SLOTS[45].as_ptr(),
        0x0105_391C => SLOTS[46].as_ptr(),
        0x0171_DE0C => SLOTS[47].as_ptr(),
        0x0105_392C => SLOTS[48].as_ptr(),
        0x0105_3930 => SLOTS[49].as_ptr(),
        0x0171_DE10 => SLOTS[50].as_ptr(),
        0x0105_39D0 => SLOTS[51].as_ptr(),
        0x0105_39D4 => SLOTS[52].as_ptr(),
        0x0171_DE2C => SLOTS[53].as_ptr(),
        0x0105_3CA4 => SLOTS[54].as_ptr(),
        0x0105_3CA8 => SLOTS[55].as_ptr(),
        0x0171_DE58 => SLOTS[56].as_ptr(),
        0x0105_459C => SLOTS[57].as_ptr(),
        0x0105_45A0 => SLOTS[58].as_ptr(),
        0x0171_DE80 => SLOTS[59].as_ptr(),
        0x0105_45D4 => SLOTS[60].as_ptr(),
        0x0105_45D8 => SLOTS[61].as_ptr(),
        0x0171_DEE0 => SLOTS[62].as_ptr(),
        0x0105_45E4 => SLOTS[63].as_ptr(),
        0x0105_45E8 => SLOTS[64].as_ptr(),
        0x0171_F9A8 => SLOTS[65].as_ptr(),
        0x0105_4618 => SLOTS[66].as_ptr(),
        0x0105_461C => SLOTS[67].as_ptr(),
        0x0171_F9B0 => SLOTS[68].as_ptr(),
        0x0105_4620 => SLOTS[69].as_ptr(),
        0x0105_4624 => SLOTS[70].as_ptr(),
        0x0171_F9B8 => SLOTS[71].as_ptr(),
        0x0105_4638 => SLOTS[72].as_ptr(),
        0x0105_463C => SLOTS[73].as_ptr(),
        0x0171_F9C4 => SLOTS[74].as_ptr(),
        0x0105_46CC => SLOTS[75].as_ptr(),
        0x0105_46D0 => SLOTS[76].as_ptr(),
        0x0171_FAC8 => SLOTS[77].as_ptr(),
        0x0105_4878 => SLOTS[78].as_ptr(),
        0x0105_487C => SLOTS[79].as_ptr(),
        0x0171_FADC => SLOTS[80].as_ptr(),
        0x0105_4A20 => SLOTS[81].as_ptr(),
        0x0105_4A24 => SLOTS[82].as_ptr(),
        0x0171_FAF8 => SLOTS[83].as_ptr(),
        0x0105_4B74 => SLOTS[84].as_ptr(),
        0x0105_4B78 => SLOTS[85].as_ptr(),
        0x0171_FB00 => SLOTS[86].as_ptr(),
        0x0105_4B7C => SLOTS[87].as_ptr(),
        0x0105_4B80 => SLOTS[88].as_ptr(),
        0x0171_FB18 => SLOTS[89].as_ptr(),
        0x0105_4BA8 => SLOTS[90].as_ptr(),
        0x0105_4BAC => SLOTS[91].as_ptr(),
        0x0172_0798 => SLOTS[92].as_ptr(),
        0x0105_4CCC => SLOTS[93].as_ptr(),
        0x0105_4CD0 => SLOTS[94].as_ptr(),
        0x0172_092C => SLOTS[95].as_ptr(),
        0x0105_5050 => SLOTS[96].as_ptr(),
        0x0105_5054 => SLOTS[97].as_ptr(),
        0x0172_0F84 => SLOTS[98].as_ptr(),
        0x0105_50AC => SLOTS[99].as_ptr(),
        0x0105_50B0 => SLOTS[100].as_ptr(),
        0x0172_0F8C => SLOTS[101].as_ptr(),
        0x0105_50F8 => SLOTS[102].as_ptr(),
        0x0105_50FC => SLOTS[103].as_ptr(),
        0x0172_0F90 => SLOTS[104].as_ptr(),
        0x0105_51C8 => SLOTS[105].as_ptr(),
        0x0105_51CC => SLOTS[106].as_ptr(),
        0x0172_0F94 => SLOTS[107].as_ptr(),
        0x0105_5278 => SLOTS[108].as_ptr(),
        0x0105_527C => SLOTS[109].as_ptr(),
        0x0172_0F98 => SLOTS[110].as_ptr(),
        0x0105_52A8 => SLOTS[111].as_ptr(),
        0x0105_52AC => SLOTS[112].as_ptr(),
        0x0172_0F9C => SLOTS[113].as_ptr(),
        0x0105_52B0 => SLOTS[114].as_ptr(),
        0x0105_52B4 => SLOTS[115].as_ptr(),
        0x0172_0FD4 => SLOTS[116].as_ptr(),
        0x0105_5378 => SLOTS[117].as_ptr(),
        0x0105_537C => SLOTS[118].as_ptr(),
        0x0172_0FE0 => SLOTS[119].as_ptr(),
        0x0105_53BC => SLOTS[120].as_ptr(),
        0x0105_53C0 => SLOTS[121].as_ptr(),
        0x0172_0FE4 => SLOTS[122].as_ptr(),
        0x0105_53F4 => SLOTS[123].as_ptr(),
        0x0105_53F8 => SLOTS[124].as_ptr(),
        0x0172_0FEC => SLOTS[125].as_ptr(),
        0x0105_5430 => SLOTS[126].as_ptr(),
        0x0105_5434 => SLOTS[127].as_ptr(),
        0x0172_0FF0 => SLOTS[128].as_ptr(),
        0x0105_5494 => SLOTS[129].as_ptr(),
        0x0105_5498 => SLOTS[130].as_ptr(),
        0x0172_0FF4 => SLOTS[131].as_ptr(),
        0x0105_55AC => SLOTS[132].as_ptr(),
        0x0105_55B0 => SLOTS[133].as_ptr(),
        0x0172_3B84 => SLOTS[134].as_ptr(),
        0x0105_5600 => SLOTS[135].as_ptr(),
        0x0105_5604 => SLOTS[136].as_ptr(),
        0x0172_3B88 => SLOTS[137].as_ptr(),
        0x0105_5668 => SLOTS[138].as_ptr(),
        0x0105_566C => SLOTS[139].as_ptr(),
        0x0172_3B90 => SLOTS[140].as_ptr(),
        0x0105_5694 => SLOTS[141].as_ptr(),
        0x0105_5698 => SLOTS[142].as_ptr(),
        0x0172_3B94 => SLOTS[143].as_ptr(),
        0x0105_56D0 => SLOTS[144].as_ptr(),
        0x0105_56D4 => SLOTS[145].as_ptr(),
        0x0172_3B98 => SLOTS[146].as_ptr(),
        0x0105_571C => SLOTS[147].as_ptr(),
        0x0105_5720 => SLOTS[148].as_ptr(),
        0x0172_3BA0 => SLOTS[149].as_ptr(),
        0x0105_5744 => SLOTS[150].as_ptr(),
        0x0105_5748 => SLOTS[151].as_ptr(),
        0x0172_3BAC => SLOTS[152].as_ptr(),
        0x0105_5774 => SLOTS[153].as_ptr(),
        0x0105_5778 => SLOTS[154].as_ptr(),
        0x0172_3BD4 => SLOTS[155].as_ptr(),
        0x0105_5798 => SLOTS[156].as_ptr(),
        0x0105_579C => SLOTS[157].as_ptr(),
        0x0172_BD18 => SLOTS[158].as_ptr(),
        0x0105_633C => SLOTS[159].as_ptr(),
        0x0105_6340 => SLOTS[160].as_ptr(),
        0x0173_0320 => SLOTS[161].as_ptr(),
        0x0105_6370 => SLOTS[162].as_ptr(),
        0x0105_6374 => SLOTS[163].as_ptr(),
        0x0173_032C => SLOTS[164].as_ptr(),
        0x0105_6430 => SLOTS[165].as_ptr(),
        0x0105_6434 => SLOTS[166].as_ptr(),
        0x0174_75EC => SLOTS[167].as_ptr(),
        _ => panic!("unexpected global VA {file_va:#x}"),
    }
}

/// Pointer to the global at a file VA, mirroring
/// `lf-checker-rt::global`.
///
/// # Panics
///
/// As [`slot_for`].
#[must_use]
pub fn global<T>(file_va: u32) -> *mut T {
    slot_for(file_va) as *mut T
}

/// Plants `bits` at the global `file_va`.
///
/// # Panics
///
/// As [`slot_for`].
pub fn plant(file_va: u32, bits: u32) {
    unsafe {
        global::<u32>(file_va).write(bits);
    }
}

/// Reads back the word at `file_va` through a fresh raw pointer, so the
/// compiler cannot forward a value the rewrite overwrote.
///
/// # Panics
///
/// As [`slot_for`].
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
    (stdcall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (stdcall).
        #[unsafe(no_mangle)]
        pub extern "stdcall" fn $name($($arg : $ty),*) -> $ret $body
    };
    (thiscall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (thiscall).
        #[unsafe(no_mangle)]
        pub extern "thiscall" fn $name($($arg : $ty),*) -> $ret $body
    };
    (fastcall, $name:ident ($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty $body:block) => {
        /// Rewrite export, called by the differential test by name (fastcall).
        #[unsafe(no_mangle)]
        pub extern "fastcall" fn $name($($arg : $ty),*) -> $ret $body
    };
}
