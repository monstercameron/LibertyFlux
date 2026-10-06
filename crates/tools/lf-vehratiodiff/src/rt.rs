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
static SLOTS: [AtomicU32; 171] = [
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
        0x0103_CE3C => SLOTS[0].as_ptr(),
        0x0103_CE40 => SLOTS[1].as_ptr(),
        0x012F_A738 => SLOTS[2].as_ptr(),
        0x0103_DB80 => SLOTS[3].as_ptr(),
        0x0103_DB84 => SLOTS[4].as_ptr(),
        0x012F_AE80 => SLOTS[5].as_ptr(),
        0x0103_DBD4 => SLOTS[6].as_ptr(),
        0x0103_DBD8 => SLOTS[7].as_ptr(),
        0x012F_B1A8 => SLOTS[8].as_ptr(),
        0x0103_DBDC => SLOTS[9].as_ptr(),
        0x0103_DBE0 => SLOTS[10].as_ptr(),
        0x012F_B1B0 => SLOTS[11].as_ptr(),
        0x0103_E468 => SLOTS[12].as_ptr(),
        0x0103_E46C => SLOTS[13].as_ptr(),
        0x012F_B1B4 => SLOTS[14].as_ptr(),
        0x0103_E48C => SLOTS[15].as_ptr(),
        0x0103_E490 => SLOTS[16].as_ptr(),
        0x012F_B1C0 => SLOTS[17].as_ptr(),
        0x0103_E83C => SLOTS[18].as_ptr(),
        0x0103_E840 => SLOTS[19].as_ptr(),
        0x012F_B1D8 => SLOTS[20].as_ptr(),
        0x0103_E864 => SLOTS[21].as_ptr(),
        0x0103_E868 => SLOTS[22].as_ptr(),
        0x012F_B1FC => SLOTS[23].as_ptr(),
        0x0103_E86C => SLOTS[24].as_ptr(),
        0x0103_E870 => SLOTS[25].as_ptr(),
        0x012F_B22C => SLOTS[26].as_ptr(),
        0x0103_E8A4 => SLOTS[27].as_ptr(),
        0x0103_E8A8 => SLOTS[28].as_ptr(),
        0x012F_B270 => SLOTS[29].as_ptr(),
        0x0103_E8C0 => SLOTS[30].as_ptr(),
        0x0103_E8C4 => SLOTS[31].as_ptr(),
        0x012F_B398 => SLOTS[32].as_ptr(),
        0x0103_E97C => SLOTS[33].as_ptr(),
        0x0103_E980 => SLOTS[34].as_ptr(),
        0x0130_5D18 => SLOTS[35].as_ptr(),
        0x0103_ED08 => SLOTS[36].as_ptr(),
        0x0103_ED0C => SLOTS[37].as_ptr(),
        0x0130_5D20 => SLOTS[38].as_ptr(),
        0x0103_ED10 => SLOTS[39].as_ptr(),
        0x0103_ED14 => SLOTS[40].as_ptr(),
        0x0139_C270 => SLOTS[41].as_ptr(),
        0x0103_ED64 => SLOTS[42].as_ptr(),
        0x0103_ED68 => SLOTS[43].as_ptr(),
        0x013B_0EA0 => SLOTS[44].as_ptr(),
        0x0103_ED70 => SLOTS[45].as_ptr(),
        0x0103_ED74 => SLOTS[46].as_ptr(),
        0x013B_0EE8 => SLOTS[47].as_ptr(),
        0x0103_ED78 => SLOTS[48].as_ptr(),
        0x0103_ED7C => SLOTS[49].as_ptr(),
        0x013B_AB30 => SLOTS[50].as_ptr(),
        0x0103_ED80 => SLOTS[51].as_ptr(),
        0x0103_ED84 => SLOTS[52].as_ptr(),
        0x013B_AB98 => SLOTS[53].as_ptr(),
        0x0103_ED90 => SLOTS[54].as_ptr(),
        0x0103_ED94 => SLOTS[55].as_ptr(),
        0x0150_E0CC => SLOTS[56].as_ptr(),
        0x0103_ED9C => SLOTS[57].as_ptr(),
        0x0103_EDA0 => SLOTS[58].as_ptr(),
        0x0150_E134 => SLOTS[59].as_ptr(),
        0x0103_F364 => SLOTS[60].as_ptr(),
        0x0103_F368 => SLOTS[61].as_ptr(),
        0x0154_E15C => SLOTS[62].as_ptr(),
        0x0103_F44C => SLOTS[63].as_ptr(),
        0x0103_F450 => SLOTS[64].as_ptr(),
        0x0154_E280 => SLOTS[65].as_ptr(),
        0x0103_F4C0 => SLOTS[66].as_ptr(),
        0x0103_F4C4 => SLOTS[67].as_ptr(),
        0x0158_D5F8 => SLOTS[68].as_ptr(),
        0x0103_F538 => SLOTS[69].as_ptr(),
        0x0103_F53C => SLOTS[70].as_ptr(),
        0x0159_3304 => SLOTS[71].as_ptr(),
        0x0103_F680 => SLOTS[72].as_ptr(),
        0x0103_F684 => SLOTS[73].as_ptr(),
        0x0159_3B60 => SLOTS[74].as_ptr(),
        0x0103_F694 => SLOTS[75].as_ptr(),
        0x0103_F698 => SLOTS[76].as_ptr(),
        0x0159_3BAC => SLOTS[77].as_ptr(),
        0x0103_F700 => SLOTS[78].as_ptr(),
        0x0103_F704 => SLOTS[79].as_ptr(),
        0x015C_15C8 => SLOTS[80].as_ptr(),
        0x0103_F970 => SLOTS[81].as_ptr(),
        0x0103_F974 => SLOTS[82].as_ptr(),
        0x015D_BDD0 => SLOTS[83].as_ptr(),
        0x0103_F99C => SLOTS[84].as_ptr(),
        0x0103_F9A0 => SLOTS[85].as_ptr(),
        0x015E_88E4 => SLOTS[86].as_ptr(),
        0x0103_FC78 => SLOTS[87].as_ptr(),
        0x0103_FC7C => SLOTS[88].as_ptr(),
        0x015F_8BA8 => SLOTS[89].as_ptr(),
        0x0103_FF34 => SLOTS[90].as_ptr(),
        0x0103_FF3C => SLOTS[91].as_ptr(),
        0x015F_8BB4 => SLOTS[92].as_ptr(),
        0x0103_FFF8 => SLOTS[93].as_ptr(),
        0x0103_FFFC => SLOTS[94].as_ptr(),
        0x0160_0FA0 => SLOTS[95].as_ptr(),
        0x0104_000C => SLOTS[96].as_ptr(),
        0x0104_0010 => SLOTS[97].as_ptr(),
        0x0160_108C => SLOTS[98].as_ptr(),
        0x0104_0084 => SLOTS[99].as_ptr(),
        0x0104_0088 => SLOTS[100].as_ptr(),
        0x0161_54B4 => SLOTS[101].as_ptr(),
        0x0104_00AC => SLOTS[102].as_ptr(),
        0x0104_00B0 => SLOTS[103].as_ptr(),
        0x0163_2BF8 => SLOTS[104].as_ptr(),
        0x0104_00E8 => SLOTS[105].as_ptr(),
        0x0104_00EC => SLOTS[106].as_ptr(),
        0x0163_2C14 => SLOTS[107].as_ptr(),
        0x0104_00F8 => SLOTS[108].as_ptr(),
        0x0104_00FC => SLOTS[109].as_ptr(),
        0x0163_349C => SLOTS[110].as_ptr(),
        0x0104_01B8 => SLOTS[111].as_ptr(),
        0x0104_01BC => SLOTS[112].as_ptr(),
        0x0163_34C4 => SLOTS[113].as_ptr(),
        0x0104_553C => SLOTS[114].as_ptr(),
        0x0104_5540 => SLOTS[115].as_ptr(),
        0x0163_3804 => SLOTS[116].as_ptr(),
        0x0104_5570 => SLOTS[117].as_ptr(),
        0x0104_5574 => SLOTS[118].as_ptr(),
        0x0164_BA90 => SLOTS[119].as_ptr(),
        0x0104_57B0 => SLOTS[120].as_ptr(),
        0x0104_57B4 => SLOTS[121].as_ptr(),
        0x0165_7638 => SLOTS[122].as_ptr(),
        0x0104_57C0 => SLOTS[123].as_ptr(),
        0x0104_57C4 => SLOTS[124].as_ptr(),
        0x0165_763C => SLOTS[125].as_ptr(),
        0x0104_57CC => SLOTS[126].as_ptr(),
        0x0104_57D0 => SLOTS[127].as_ptr(),
        0x0165_7644 => SLOTS[128].as_ptr(),
        0x0104_57F4 => SLOTS[129].as_ptr(),
        0x0104_57F8 => SLOTS[130].as_ptr(),
        0x0165_FB94 => SLOTS[131].as_ptr(),
        0x0104_57FC => SLOTS[132].as_ptr(),
        0x0104_5800 => SLOTS[133].as_ptr(),
        0x0165_FD20 => SLOTS[134].as_ptr(),
        0x0104_5804 => SLOTS[135].as_ptr(),
        0x0104_5808 => SLOTS[136].as_ptr(),
        0x0166_0300 => SLOTS[137].as_ptr(),
        0x0104_58A8 => SLOTS[138].as_ptr(),
        0x0104_58AC => SLOTS[139].as_ptr(),
        0x0166_140C => SLOTS[140].as_ptr(),
        0x0104_59C0 => SLOTS[141].as_ptr(),
        0x0104_59C4 => SLOTS[142].as_ptr(),
        0x0166_24CC => SLOTS[143].as_ptr(),
        0x0104_59E0 => SLOTS[144].as_ptr(),
        0x0104_59E4 => SLOTS[145].as_ptr(),
        0x0166_5424 => SLOTS[146].as_ptr(),
        0x0104_5A18 => SLOTS[147].as_ptr(),
        0x0104_5A1C => SLOTS[148].as_ptr(),
        0x0166_69CC => SLOTS[149].as_ptr(),
        0x0104_6050 => SLOTS[150].as_ptr(),
        0x0104_6054 => SLOTS[151].as_ptr(),
        0x0166_8388 => SLOTS[152].as_ptr(),
        0x0104_6058 => SLOTS[153].as_ptr(),
        0x0104_605C => SLOTS[154].as_ptr(),
        0x0166_8390 => SLOTS[155].as_ptr(),
        0x0105_0938 => SLOTS[156].as_ptr(),
        0x0105_093C => SLOTS[157].as_ptr(),
        0x016F_C620 => SLOTS[158].as_ptr(),
        0x0105_0A58 => SLOTS[159].as_ptr(),
        0x0105_0A5C => SLOTS[160].as_ptr(),
        0x016F_C630 => SLOTS[161].as_ptr(),
        0x0105_0AAC => SLOTS[162].as_ptr(),
        0x0105_0AB0 => SLOTS[163].as_ptr(),
        0x0171_3A98 => SLOTS[164].as_ptr(),
        0x0105_0AF0 => SLOTS[165].as_ptr(),
        0x0105_0AF4 => SLOTS[166].as_ptr(),
        0x0171_BB64 => SLOTS[167].as_ptr(),
        0x0105_0D1C => SLOTS[168].as_ptr(),
        0x0105_0D20 => SLOTS[169].as_ptr(),
        0x0171_BBA0 => SLOTS[170].as_ptr(),
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
