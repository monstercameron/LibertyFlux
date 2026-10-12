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
        0x01030B80 => SLOTS[0].as_ptr(),
        0x01030B84 => SLOTS[1].as_ptr(),
        0x01161514 => SLOTS[2].as_ptr(),
        0x01031910 => SLOTS[3].as_ptr(),
        0x01031914 => SLOTS[4].as_ptr(),
        0x0116BFE8 => SLOTS[5].as_ptr(),
        0x01031B18 => SLOTS[6].as_ptr(),
        0x01031B1C => SLOTS[7].as_ptr(),
        0x01173354 => SLOTS[8].as_ptr(),
        0x01032320 => SLOTS[9].as_ptr(),
        0x01032324 => SLOTS[10].as_ptr(),
        0x0117358C => SLOTS[11].as_ptr(),
        0x01032338 => SLOTS[12].as_ptr(),
        0x0103233C => SLOTS[13].as_ptr(),
        0x01173690 => SLOTS[14].as_ptr(),
        0x01032358 => SLOTS[15].as_ptr(),
        0x0103235C => SLOTS[16].as_ptr(),
        0x011736A0 => SLOTS[17].as_ptr(),
        0x0103276C => SLOTS[18].as_ptr(),
        0x01032770 => SLOTS[19].as_ptr(),
        0x01173728 => SLOTS[20].as_ptr(),
        0x0103277C => SLOTS[21].as_ptr(),
        0x01032780 => SLOTS[22].as_ptr(),
        0x0117379C => SLOTS[23].as_ptr(),
        0x01032788 => SLOTS[24].as_ptr(),
        0x0103278C => SLOTS[25].as_ptr(),
        0x01175764 => SLOTS[26].as_ptr(),
        0x01032798 => SLOTS[27].as_ptr(),
        0x0103279C => SLOTS[28].as_ptr(),
        0x01175C54 => SLOTS[29].as_ptr(),
        0x01032F60 => SLOTS[30].as_ptr(),
        0x01032F64 => SLOTS[31].as_ptr(),
        0x011764C8 => SLOTS[32].as_ptr(),
        0x01032F70 => SLOTS[33].as_ptr(),
        0x01032F74 => SLOTS[34].as_ptr(),
        0x01176870 => SLOTS[35].as_ptr(),
        0x01032FE4 => SLOTS[36].as_ptr(),
        0x01033050 => SLOTS[37].as_ptr(),
        0x01176D40 => SLOTS[38].as_ptr(),
        0x010330C0 => SLOTS[39].as_ptr(),
        0x010330C4 => SLOTS[40].as_ptr(),
        0x0117968C => SLOTS[41].as_ptr(),
        0x010330F0 => SLOTS[42].as_ptr(),
        0x010330F4 => SLOTS[43].as_ptr(),
        0x0117E6C4 => SLOTS[44].as_ptr(),
        0x01033100 => SLOTS[45].as_ptr(),
        0x01033104 => SLOTS[46].as_ptr(),
        0x0117E6FC => SLOTS[47].as_ptr(),
        0x0103310C => SLOTS[48].as_ptr(),
        0x01033110 => SLOTS[49].as_ptr(),
        0x0118D7E4 => SLOTS[50].as_ptr(),
        0x01033114 => SLOTS[51].as_ptr(),
        0x01033118 => SLOTS[52].as_ptr(),
        0x0118DEBC => SLOTS[53].as_ptr(),
        0x01033128 => SLOTS[54].as_ptr(),
        0x0103312C => SLOTS[55].as_ptr(),
        0x0118E7DC => SLOTS[56].as_ptr(),
        0x01034468 => SLOTS[57].as_ptr(),
        0x0103446C => SLOTS[58].as_ptr(),
        0x0118E924 => SLOTS[59].as_ptr(),
        0x01034488 => SLOTS[60].as_ptr(),
        0x0103448C => SLOTS[61].as_ptr(),
        0x01190E68 => SLOTS[62].as_ptr(),
        0x01034664 => SLOTS[63].as_ptr(),
        0x0103668C => SLOTS[64].as_ptr(),
        0x0119BF10 => SLOTS[65].as_ptr(),
        0x01036778 => SLOTS[66].as_ptr(),
        0x0103677C => SLOTS[67].as_ptr(),
        0x011A0B00 => SLOTS[68].as_ptr(),
        0x01036AD4 => SLOTS[69].as_ptr(),
        0x01036AD8 => SLOTS[70].as_ptr(),
        0x011A1BE4 => SLOTS[71].as_ptr(),
        0x01036B84 => SLOTS[72].as_ptr(),
        0x01036B88 => SLOTS[73].as_ptr(),
        0x011A28F8 => SLOTS[74].as_ptr(),
        0x01036BDC => SLOTS[75].as_ptr(),
        0x01036BE0 => SLOTS[76].as_ptr(),
        0x011A2A68 => SLOTS[77].as_ptr(),
        0x01036E88 => SLOTS[78].as_ptr(),
        0x01036E8C => SLOTS[79].as_ptr(),
        0x011A2E90 => SLOTS[80].as_ptr(),
        0x01036EA8 => SLOTS[81].as_ptr(),
        0x01036EAC => SLOTS[82].as_ptr(),
        0x011A2EB4 => SLOTS[83].as_ptr(),
        0x01036EFC => SLOTS[84].as_ptr(),
        0x01036F00 => SLOTS[85].as_ptr(),
        0x011A4F1C => SLOTS[86].as_ptr(),
        0x01036F04 => SLOTS[87].as_ptr(),
        0x01036F08 => SLOTS[88].as_ptr(),
        0x011A4FA4 => SLOTS[89].as_ptr(),
        0x01038A28 => SLOTS[90].as_ptr(),
        0x01038A2C => SLOTS[91].as_ptr(),
        0x01238838 => SLOTS[92].as_ptr(),
        0x01038A58 => SLOTS[93].as_ptr(),
        0x01038A5C => SLOTS[94].as_ptr(),
        0x012388F0 => SLOTS[95].as_ptr(),
        0x01038A9C => SLOTS[96].as_ptr(),
        0x01038AA0 => SLOTS[97].as_ptr(),
        0x01282F40 => SLOTS[98].as_ptr(),
        0x01038B10 => SLOTS[99].as_ptr(),
        0x01038B14 => SLOTS[100].as_ptr(),
        0x0128304C => SLOTS[101].as_ptr(),
        0x01038B40 => SLOTS[102].as_ptr(),
        0x01038B44 => SLOTS[103].as_ptr(),
        0x01283204 => SLOTS[104].as_ptr(),
        0x01038C5C => SLOTS[105].as_ptr(),
        0x01038C64 => SLOTS[106].as_ptr(),
        0x01283F30 => SLOTS[107].as_ptr(),
        0x01038E14 => SLOTS[108].as_ptr(),
        0x01038E18 => SLOTS[109].as_ptr(),
        0x01284574 => SLOTS[110].as_ptr(),
        0x01039040 => SLOTS[111].as_ptr(),
        0x01039044 => SLOTS[112].as_ptr(),
        0x01288778 => SLOTS[113].as_ptr(),
        0x01039144 => SLOTS[114].as_ptr(),
        0x01039148 => SLOTS[115].as_ptr(),
        0x0128A8D8 => SLOTS[116].as_ptr(),
        0x010391E4 => SLOTS[117].as_ptr(),
        0x010391E8 => SLOTS[118].as_ptr(),
        0x0128AA88 => SLOTS[119].as_ptr(),
        0x0103928C => SLOTS[120].as_ptr(),
        0x01039290 => SLOTS[121].as_ptr(),
        0x0128E3F0 => SLOTS[122].as_ptr(),
        0x0103AB84 => SLOTS[123].as_ptr(),
        0x0103AB88 => SLOTS[124].as_ptr(),
        0x0128E958 => SLOTS[125].as_ptr(),
        0x0103AC0C => SLOTS[126].as_ptr(),
        0x0103AC10 => SLOTS[127].as_ptr(),
        0x01292448 => SLOTS[128].as_ptr(),
        0x0103AC50 => SLOTS[129].as_ptr(),
        0x0103AC54 => SLOTS[130].as_ptr(),
        0x01295794 => SLOTS[131].as_ptr(),
        0x0103ACAC => SLOTS[132].as_ptr(),
        0x0103ACB0 => SLOTS[133].as_ptr(),
        0x01295860 => SLOTS[134].as_ptr(),
        0x0103ACEC => SLOTS[135].as_ptr(),
        0x0103ACF0 => SLOTS[136].as_ptr(),
        0x01295870 => SLOTS[137].as_ptr(),
        0x0103AD5C => SLOTS[138].as_ptr(),
        0x0103AD60 => SLOTS[139].as_ptr(),
        0x01295ACC => SLOTS[140].as_ptr(),
        0x0103B118 => SLOTS[141].as_ptr(),
        0x0103B11C => SLOTS[142].as_ptr(),
        0x012B41B8 => SLOTS[143].as_ptr(),
        0x0103B180 => SLOTS[144].as_ptr(),
        0x0103B184 => SLOTS[145].as_ptr(),
        0x012B6094 => SLOTS[146].as_ptr(),
        0x0103B568 => SLOTS[147].as_ptr(),
        0x0103B56C => SLOTS[148].as_ptr(),
        0x012B6254 => SLOTS[149].as_ptr(),
        0x0103B570 => SLOTS[150].as_ptr(),
        0x0103B574 => SLOTS[151].as_ptr(),
        0x012B6494 => SLOTS[152].as_ptr(),
        0x0103B5A0 => SLOTS[153].as_ptr(),
        0x0103B5A4 => SLOTS[154].as_ptr(),
        0x012B90F8 => SLOTS[155].as_ptr(),
        0x0103B688 => SLOTS[156].as_ptr(),
        0x0103B68C => SLOTS[157].as_ptr(),
        0x012BCCCC => SLOTS[158].as_ptr(),
        0x0103B6D4 => SLOTS[159].as_ptr(),
        0x0103B6D8 => SLOTS[160].as_ptr(),
        0x012BD098 => SLOTS[161].as_ptr(),
        0x0103B6E4 => SLOTS[162].as_ptr(),
        0x0103B6E8 => SLOTS[163].as_ptr(),
        0x012BD0D8 => SLOTS[164].as_ptr(),
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
