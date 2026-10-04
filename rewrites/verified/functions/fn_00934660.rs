// original: 0x00934660 collect_id_mask32
/// 0x00934660: OR-accumulate the per-record id bytes (+0x229, gated on
/// +0x226) of every record into a 32-bit mask (bit v mod 32). Ids of 0
/// contribute nothing.
// shared layout constant
const SCAN_STRIDE: u32 = 0x22c;
// shared layout constant
const SET_COUNT: usize = 0x148;
// shared layout constant
const SET_RECORDS: usize = 0x144;
export!(thiscall, rw_00934660(this: *const u8) -> u32 {
    callee_thiscall!(1, u32, this as u32);
    callee_thiscall!(2, u32, this as u32);
    let count =
        unsafe { core::ptr::read_unaligned(this.add(SET_COUNT) as *const u16) } as i32;
    let mut acc = 0u32;
    if count > 0 {
        let base =
            unsafe { core::ptr::read_unaligned(this.add(SET_RECORDS) as *const u32) };
        for i in 0..count {
            let rec = base.wrapping_add((i as u32).wrapping_mul(SCAN_STRIDE));
            unsafe {
                if core::ptr::read(rec.wrapping_add(0x226) as *const u8) == 0 {
                    continue;
                }
                let v = core::ptr::read(rec.wrapping_add(0x229) as *const u8);
                if v != 0 {
                    acc |= 1u32.wrapping_shl(v as u32);
                }
            }
        }
    }
    acc
});
