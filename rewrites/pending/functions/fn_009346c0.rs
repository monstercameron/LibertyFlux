// original: 0x009346c0 find_slot_by_id
/// 0x009346C0: find the first record whose id byte (+0x228) equals the
/// target (compared as a full word, so only targets below 0x100 can hit).
/// Returns the record index, or -1 when no record matches.
// shared layout constant
const SCAN_STRIDE: u32 = 0x22c;
// shared layout constant
const SET_COUNT: usize = 0x148;
// shared layout constant
const SET_RECORDS: usize = 0x144;
export!(thiscall, rw_009346c0(this: *const u8, target: u32) -> u32 {
    let count =
        unsafe { core::ptr::read_unaligned(this.add(SET_COUNT) as *const u16) } as i32;
    if count > 0 {
        let base =
            unsafe { core::ptr::read_unaligned(this.add(SET_RECORDS) as *const u32) };
        for i in 0..count {
            let rec = base.wrapping_add((i as u32).wrapping_mul(SCAN_STRIDE));
            let v =
                unsafe { core::ptr::read(rec.wrapping_add(0x228) as *const u8) } as u32;
            if v == target {
                return i as u32;
            }
        }
    }
    0xffff_ffff
});
