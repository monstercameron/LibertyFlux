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
        0x0103B744 => SLOTS[0].as_ptr(),
        0x0103B748 => SLOTS[1].as_ptr(),
        0x012BD17C => SLOTS[2].as_ptr(),
        0x0103B918 => SLOTS[3].as_ptr(),
        0x0103B91C => SLOTS[4].as_ptr(),
        0x012BD1D4 => SLOTS[5].as_ptr(),
        0x0103C04C => SLOTS[6].as_ptr(),
        0x0103C050 => SLOTS[7].as_ptr(),
        0x012DD2A8 => SLOTS[8].as_ptr(),
        0x0103C0D0 => SLOTS[9].as_ptr(),
        0x0103C0D4 => SLOTS[10].as_ptr(),
        0x012DD5F0 => SLOTS[11].as_ptr(),
        0x0103C7AC => SLOTS[12].as_ptr(),
        0x0103C7B0 => SLOTS[13].as_ptr(),
        0x012DD638 => SLOTS[14].as_ptr(),
        0x0103C868 => SLOTS[15].as_ptr(),
        0x0103C86C => SLOTS[16].as_ptr(),
        0x012DD8D8 => SLOTS[17].as_ptr(),
        0x0103C870 => SLOTS[18].as_ptr(),
        0x0103C874 => SLOTS[19].as_ptr(),
        0x012DDEF8 => SLOTS[20].as_ptr(),
        0x0103CA2C => SLOTS[21].as_ptr(),
        0x0103CA30 => SLOTS[22].as_ptr(),
        0x012DDF00 => SLOTS[23].as_ptr(),
        0x0103CAF8 => SLOTS[24].as_ptr(),
        0x0103CAFC => SLOTS[25].as_ptr(),
        0x012DDF18 => SLOTS[26].as_ptr(),
        0x0103CB1C => SLOTS[27].as_ptr(),
        0x0103CB20 => SLOTS[28].as_ptr(),
        0x012E21E0 => SLOTS[29].as_ptr(),
        0x0103CC88 => SLOTS[30].as_ptr(),
        0x0103CC8C => SLOTS[31].as_ptr(),
        0x012E23B0 => SLOTS[32].as_ptr(),
        0x0103CD1C => SLOTS[33].as_ptr(),
        0x0103CD20 => SLOTS[34].as_ptr(),
        0x012F4530 => SLOTS[35].as_ptr(),
        0x0103CD24 => SLOTS[36].as_ptr(),
        0x0103CD28 => SLOTS[37].as_ptr(),
        0x012F5EE0 => SLOTS[38].as_ptr(),
        0x0103CD34 => SLOTS[39].as_ptr(),
        0x0103CD38 => SLOTS[40].as_ptr(),
        0x012F7308 => SLOTS[41].as_ptr(),
        0x0103CDC8 => SLOTS[42].as_ptr(),
        0x0103CDCC => SLOTS[43].as_ptr(),
        0x012F8340 => SLOTS[44].as_ptr(),
        0x0103CDD0 => SLOTS[45].as_ptr(),
        0x0103CDD4 => SLOTS[46].as_ptr(),
        0x012F9D38 => SLOTS[47].as_ptr(),
        0x01046084 => SLOTS[48].as_ptr(),
        0x01046088 => SLOTS[49].as_ptr(),
        0x01668398 => SLOTS[50].as_ptr(),
        0x010460C8 => SLOTS[51].as_ptr(),
        0x010460CC => SLOTS[52].as_ptr(),
        0x01668DA0 => SLOTS[53].as_ptr(),
        0x010460D0 => SLOTS[54].as_ptr(),
        0x010460D4 => SLOTS[55].as_ptr(),
        0x01669D10 => SLOTS[56].as_ptr(),
        0x010460DC => SLOTS[57].as_ptr(),
        0x010460E0 => SLOTS[58].as_ptr(),
        0x0166D9E8 => SLOTS[59].as_ptr(),
        0x01046348 => SLOTS[60].as_ptr(),
        0x0104634C => SLOTS[61].as_ptr(),
        0x0166D9F0 => SLOTS[62].as_ptr(),
        0x0104636C => SLOTS[63].as_ptr(),
        0x01046370 => SLOTS[64].as_ptr(),
        0x0166D9F8 => SLOTS[65].as_ptr(),
        0x01046538 => SLOTS[66].as_ptr(),
        0x0104653C => SLOTS[67].as_ptr(),
        0x0166DA00 => SLOTS[68].as_ptr(),
        0x01046878 => SLOTS[69].as_ptr(),
        0x0104687C => SLOTS[70].as_ptr(),
        0x0166DAF0 => SLOTS[71].as_ptr(),
        0x010469A4 => SLOTS[72].as_ptr(),
        0x010469A8 => SLOTS[73].as_ptr(),
        0x0166FED8 => SLOTS[74].as_ptr(),
        0x01046A70 => SLOTS[75].as_ptr(),
        0x01046A74 => SLOTS[76].as_ptr(),
        0x01670510 => SLOTS[77].as_ptr(),
        0x01046ABC => SLOTS[78].as_ptr(),
        0x01046AC0 => SLOTS[79].as_ptr(),
        0x01670538 => SLOTS[80].as_ptr(),
        0x01046AD4 => SLOTS[81].as_ptr(),
        0x01046AD8 => SLOTS[82].as_ptr(),
        0x01670548 => SLOTS[83].as_ptr(),
        0x01046B3C => SLOTS[84].as_ptr(),
        0x01046B40 => SLOTS[85].as_ptr(),
        0x01670CD8 => SLOTS[86].as_ptr(),
        0x01046ED0 => SLOTS[87].as_ptr(),
        0x01046ED4 => SLOTS[88].as_ptr(),
        0x0167CB14 => SLOTS[89].as_ptr(),
        0x01046EE8 => SLOTS[90].as_ptr(),
        0x01046EEC => SLOTS[91].as_ptr(),
        0x0167CCB4 => SLOTS[92].as_ptr(),
        0x01046EF0 => SLOTS[93].as_ptr(),
        0x01046EF4 => SLOTS[94].as_ptr(),
        0x0167CCD0 => SLOTS[95].as_ptr(),
        0x01046F14 => SLOTS[96].as_ptr(),
        0x01046F18 => SLOTS[97].as_ptr(),
        0x0167E2A4 => SLOTS[98].as_ptr(),
        0x01046F30 => SLOTS[99].as_ptr(),
        0x01046F34 => SLOTS[100].as_ptr(),
        0x0167E2B0 => SLOTS[101].as_ptr(),
        0x01047200 => SLOTS[102].as_ptr(),
        0x01047204 => SLOTS[103].as_ptr(),
        0x0167E2B4 => SLOTS[104].as_ptr(),
        0x01047244 => SLOTS[105].as_ptr(),
        0x01047248 => SLOTS[106].as_ptr(),
        0x0167E2D0 => SLOTS[107].as_ptr(),
        0x01047264 => SLOTS[108].as_ptr(),
        0x01047268 => SLOTS[109].as_ptr(),
        0x0167E2D4 => SLOTS[110].as_ptr(),
        0x010472EC => SLOTS[111].as_ptr(),
        0x010472F0 => SLOTS[112].as_ptr(),
        0x0167E3B8 => SLOTS[113].as_ptr(),
        0x010472F4 => SLOTS[114].as_ptr(),
        0x010472F8 => SLOTS[115].as_ptr(),
        0x0167E3C0 => SLOTS[116].as_ptr(),
        0x01047320 => SLOTS[117].as_ptr(),
        0x01047324 => SLOTS[118].as_ptr(),
        0x0167E3C4 => SLOTS[119].as_ptr(),
        0x01047328 => SLOTS[120].as_ptr(),
        0x0104732C => SLOTS[121].as_ptr(),
        0x0167F004 => SLOTS[122].as_ptr(),
        0x01047330 => SLOTS[123].as_ptr(),
        0x01047334 => SLOTS[124].as_ptr(),
        0x0167F090 => SLOTS[125].as_ptr(),
        0x01047358 => SLOTS[126].as_ptr(),
        0x0104735C => SLOTS[127].as_ptr(),
        0x0167F094 => SLOTS[128].as_ptr(),
        0x01047360 => SLOTS[129].as_ptr(),
        0x01047364 => SLOTS[130].as_ptr(),
        0x0167F0AC => SLOTS[131].as_ptr(),
        0x01047370 => SLOTS[132].as_ptr(),
        0x01047374 => SLOTS[133].as_ptr(),
        0x0167F100 => SLOTS[134].as_ptr(),
        0x01047378 => SLOTS[135].as_ptr(),
        0x0104737C => SLOTS[136].as_ptr(),
        0x0167F11C => SLOTS[137].as_ptr(),
        0x01047580 => SLOTS[138].as_ptr(),
        0x01047584 => SLOTS[139].as_ptr(),
        0x0167F130 => SLOTS[140].as_ptr(),
        0x01047588 => SLOTS[141].as_ptr(),
        0x0104758C => SLOTS[142].as_ptr(),
        0x0167F144 => SLOTS[143].as_ptr(),
        0x01047590 => SLOTS[144].as_ptr(),
        0x01047594 => SLOTS[145].as_ptr(),
        0x0167F168 => SLOTS[146].as_ptr(),
        0x01047598 => SLOTS[147].as_ptr(),
        0x0104759C => SLOTS[148].as_ptr(),
        0x0167F188 => SLOTS[149].as_ptr(),
        0x010475B0 => SLOTS[150].as_ptr(),
        0x010475B4 => SLOTS[151].as_ptr(),
        0x0167F590 => SLOTS[152].as_ptr(),
        0x010475C8 => SLOTS[153].as_ptr(),
        0x010475CC => SLOTS[154].as_ptr(),
        0x0167F634 => SLOTS[155].as_ptr(),
        0x01047A28 => SLOTS[156].as_ptr(),
        0x01047A2C => SLOTS[157].as_ptr(),
        0x01682F24 => SLOTS[158].as_ptr(),
        0x01047A30 => SLOTS[159].as_ptr(),
        0x01047A34 => SLOTS[160].as_ptr(),
        0x01682F30 => SLOTS[161].as_ptr(),
        0x01047A90 => SLOTS[162].as_ptr(),
        0x01047A94 => SLOTS[163].as_ptr(),
        0x01682F68 => SLOTS[164].as_ptr(),
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
