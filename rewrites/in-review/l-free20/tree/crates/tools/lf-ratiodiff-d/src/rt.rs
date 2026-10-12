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
static SLOTS: [AtomicU32; 165] = [AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0)];

/// The slot behind a file address. The address table is generated from the
/// rewrites' own global operands; an unlisted address is a test bug.
fn slot_for(file_va: u32) -> *const AtomicU32 {
    match file_va {
        0x01057680 => SLOTS[0].as_ptr(),
        0x01057684 => SLOTS[1].as_ptr(),
        0x017A65B0 => SLOTS[2].as_ptr(),
        0x01057688 => SLOTS[3].as_ptr(),
        0x0105768C => SLOTS[4].as_ptr(),
        0x017A6678 => SLOTS[5].as_ptr(),
        0x010576BC => SLOTS[6].as_ptr(),
        0x010576C0 => SLOTS[7].as_ptr(),
        0x017A667C => SLOTS[8].as_ptr(),
        0x010576C4 => SLOTS[9].as_ptr(),
        0x010576C8 => SLOTS[10].as_ptr(),
        0x017A6680 => SLOTS[11].as_ptr(),
        0x010576E8 => SLOTS[12].as_ptr(),
        0x010576EC => SLOTS[13].as_ptr(),
        0x017A6684 => SLOTS[14].as_ptr(),
        0x010576F0 => SLOTS[15].as_ptr(),
        0x010576F4 => SLOTS[16].as_ptr(),
        0x017A6688 => SLOTS[17].as_ptr(),
        0x010576F8 => SLOTS[18].as_ptr(),
        0x010576FC => SLOTS[19].as_ptr(),
        0x017A668C => SLOTS[20].as_ptr(),
        0x0105771C => SLOTS[21].as_ptr(),
        0x01057720 => SLOTS[22].as_ptr(),
        0x017A6694 => SLOTS[23].as_ptr(),
        0x01057760 => SLOTS[24].as_ptr(),
        0x01057764 => SLOTS[25].as_ptr(),
        0x017A669C => SLOTS[26].as_ptr(),
        0x010577D0 => SLOTS[27].as_ptr(),
        0x010577D4 => SLOTS[28].as_ptr(),
        0x017A66A0 => SLOTS[29].as_ptr(),
        0x01057820 => SLOTS[30].as_ptr(),
        0x01057824 => SLOTS[31].as_ptr(),
        0x017AA680 => SLOTS[32].as_ptr(),
        0x01057828 => SLOTS[33].as_ptr(),
        0x0105782C => SLOTS[34].as_ptr(),
        0x017AA684 => SLOTS[35].as_ptr(),
        0x01057868 => SLOTS[36].as_ptr(),
        0x0105786C => SLOTS[37].as_ptr(),
        0x017AB500 => SLOTS[38].as_ptr(),
        0x01057894 => SLOTS[39].as_ptr(),
        0x01057898 => SLOTS[40].as_ptr(),
        0x017AB508 => SLOTS[41].as_ptr(),
        0x01057978 => SLOTS[42].as_ptr(),
        0x0105797C => SLOTS[43].as_ptr(),
        0x017AB50C => SLOTS[44].as_ptr(),
        0x010579F8 => SLOTS[45].as_ptr(),
        0x010579FC => SLOTS[46].as_ptr(),
        0x017AB514 => SLOTS[47].as_ptr(),
        0x01057A78 => SLOTS[48].as_ptr(),
        0x01057A7C => SLOTS[49].as_ptr(),
        0x017AB52C => SLOTS[50].as_ptr(),
        0x01057AA4 => SLOTS[51].as_ptr(),
        0x01057AA8 => SLOTS[52].as_ptr(),
        0x017AB530 => SLOTS[53].as_ptr(),
        0x01057AFC => SLOTS[54].as_ptr(),
        0x01057B00 => SLOTS[55].as_ptr(),
        0x017AB534 => SLOTS[56].as_ptr(),
        0x01057B34 => SLOTS[57].as_ptr(),
        0x01057B38 => SLOTS[58].as_ptr(),
        0x017AB538 => SLOTS[59].as_ptr(),
        0x01057B3C => SLOTS[60].as_ptr(),
        0x01057B40 => SLOTS[61].as_ptr(),
        0x017AB53C => SLOTS[62].as_ptr(),
        0x01057B44 => SLOTS[63].as_ptr(),
        0x01057B48 => SLOTS[64].as_ptr(),
        0x017AB540 => SLOTS[65].as_ptr(),
        0x01057B90 => SLOTS[66].as_ptr(),
        0x01057B94 => SLOTS[67].as_ptr(),
        0x017AB544 => SLOTS[68].as_ptr(),
        0x01057BC0 => SLOTS[69].as_ptr(),
        0x01057BC4 => SLOTS[70].as_ptr(),
        0x017AB548 => SLOTS[71].as_ptr(),
        0x01057BE4 => SLOTS[72].as_ptr(),
        0x01057BE8 => SLOTS[73].as_ptr(),
        0x017AB54C => SLOTS[74].as_ptr(),
        0x01057C28 => SLOTS[75].as_ptr(),
        0x01057C2C => SLOTS[76].as_ptr(),
        0x017AB550 => SLOTS[77].as_ptr(),
        0x01057CC0 => SLOTS[78].as_ptr(),
        0x01057CC4 => SLOTS[79].as_ptr(),
        0x017AB55C => SLOTS[80].as_ptr(),
        0x01057CC8 => SLOTS[81].as_ptr(),
        0x01057CCC => SLOTS[82].as_ptr(),
        0x017AB560 => SLOTS[83].as_ptr(),
        0x01057CEC => SLOTS[84].as_ptr(),
        0x01057CF0 => SLOTS[85].as_ptr(),
        0x017AB564 => SLOTS[86].as_ptr(),
        0x01057D30 => SLOTS[87].as_ptr(),
        0x01057D34 => SLOTS[88].as_ptr(),
        0x017AB568 => SLOTS[89].as_ptr(),
        0x01057D70 => SLOTS[90].as_ptr(),
        0x01057D74 => SLOTS[91].as_ptr(),
        0x017AB56C => SLOTS[92].as_ptr(),
        0x01057D98 => SLOTS[93].as_ptr(),
        0x01057D9C => SLOTS[94].as_ptr(),
        0x017AB570 => SLOTS[95].as_ptr(),
        0x01057DC0 => SLOTS[96].as_ptr(),
        0x01057DC4 => SLOTS[97].as_ptr(),
        0x017AB574 => SLOTS[98].as_ptr(),
        0x01057DE8 => SLOTS[99].as_ptr(),
        0x01057DEC => SLOTS[100].as_ptr(),
        0x017AB578 => SLOTS[101].as_ptr(),
        0x01057E1C => SLOTS[102].as_ptr(),
        0x01057E20 => SLOTS[103].as_ptr(),
        0x017AB57C => SLOTS[104].as_ptr(),
        0x01057E44 => SLOTS[105].as_ptr(),
        0x01057E48 => SLOTS[106].as_ptr(),
        0x017AB580 => SLOTS[107].as_ptr(),
        0x01057E6C => SLOTS[108].as_ptr(),
        0x01057E70 => SLOTS[109].as_ptr(),
        0x017AB584 => SLOTS[110].as_ptr(),
        0x01057E90 => SLOTS[111].as_ptr(),
        0x01057E94 => SLOTS[112].as_ptr(),
        0x017AB588 => SLOTS[113].as_ptr(),
        0x01057E98 => SLOTS[114].as_ptr(),
        0x01057E9C => SLOTS[115].as_ptr(),
        0x017AB58C => SLOTS[116].as_ptr(),
        0x01057EB8 => SLOTS[117].as_ptr(),
        0x01057EBC => SLOTS[118].as_ptr(),
        0x017AB590 => SLOTS[119].as_ptr(),
        0x01057EE8 => SLOTS[120].as_ptr(),
        0x01057EEC => SLOTS[121].as_ptr(),
        0x017AB594 => SLOTS[122].as_ptr(),
        0x01057F0C => SLOTS[123].as_ptr(),
        0x01057F10 => SLOTS[124].as_ptr(),
        0x017AB598 => SLOTS[125].as_ptr(),
        0x01057F30 => SLOTS[126].as_ptr(),
        0x01057F34 => SLOTS[127].as_ptr(),
        0x017AB59C => SLOTS[128].as_ptr(),
        0x01057F5C => SLOTS[129].as_ptr(),
        0x01057F60 => SLOTS[130].as_ptr(),
        0x017AB5A0 => SLOTS[131].as_ptr(),
        0x01059560 => SLOTS[132].as_ptr(),
        0x01059564 => SLOTS[133].as_ptr(),
        0x017ACC60 => SLOTS[134].as_ptr(),
        0x01059590 => SLOTS[135].as_ptr(),
        0x01059594 => SLOTS[136].as_ptr(),
        0x017ACC64 => SLOTS[137].as_ptr(),
        0x010595A0 => SLOTS[138].as_ptr(),
        0x010595A4 => SLOTS[139].as_ptr(),
        0x017ACC68 => SLOTS[140].as_ptr(),
        0x010595E4 => SLOTS[141].as_ptr(),
        0x010595E8 => SLOTS[142].as_ptr(),
        0x017ACC6C => SLOTS[143].as_ptr(),
        0x01059604 => SLOTS[144].as_ptr(),
        0x01059608 => SLOTS[145].as_ptr(),
        0x017ACC70 => SLOTS[146].as_ptr(),
        0x01059628 => SLOTS[147].as_ptr(),
        0x0105962C => SLOTS[148].as_ptr(),
        0x017ACC74 => SLOTS[149].as_ptr(),
        0x0105964C => SLOTS[150].as_ptr(),
        0x01059650 => SLOTS[151].as_ptr(),
        0x017ACC78 => SLOTS[152].as_ptr(),
        0x010596A4 => SLOTS[153].as_ptr(),
        0x010596A8 => SLOTS[154].as_ptr(),
        0x017ACC7C => SLOTS[155].as_ptr(),
        0x010596CC => SLOTS[156].as_ptr(),
        0x010596D0 => SLOTS[157].as_ptr(),
        0x017ACC80 => SLOTS[158].as_ptr(),
        0x010596F4 => SLOTS[159].as_ptr(),
        0x010596F8 => SLOTS[160].as_ptr(),
        0x017ACCA8 => SLOTS[161].as_ptr(),
        0x010596FC => SLOTS[162].as_ptr(),
        0x01059700 => SLOTS[163].as_ptr(),
        0x017ACCAC => SLOTS[164].as_ptr(),
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
