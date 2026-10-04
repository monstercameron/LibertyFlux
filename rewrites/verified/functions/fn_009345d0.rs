// original: 0x009345d0 collect_id_mask64
/// 0x009345D0: OR-accumulate the per-record id bytes (+0x228, gated on
/// +0x226) of every record into a 64-bit mask pair (ids 0-31 into EAX,
/// ids 32-63 into EDX). Ids of 0 or >= 0x40 contribute nothing.
// shared layout constant
const SCAN_STRIDE: u32 = 0x22c;
// shared layout constant
const SET_COUNT: usize = 0x148;
// shared layout constant
const SET_RECORDS: usize = 0x144;
export!(thiscall, rw_009345d0(this: *const u8) -> u64 {
    callee_thiscall!(1, u32, this as u32);
    callee_thiscall!(2, u32, this as u32);
    let count =
        unsafe { core::ptr::read_unaligned(this.add(SET_COUNT) as *const u16) } as i32;
    let mut lo = 0u32;
    let mut hi = 0u32;
    if count > 0 {
        let base =
            unsafe { core::ptr::read_unaligned(this.add(SET_RECORDS) as *const u32) };
        for i in 0..count {
            let rec = base.wrapping_add((i as u32).wrapping_mul(SCAN_STRIDE));
            unsafe {
                if core::ptr::read(rec.wrapping_add(0x226) as *const u8) == 0 {
                    continue;
                }
                let v = core::ptr::read(rec.wrapping_add(0x228) as *const u8);
                if v == 0 {
                    continue;
                } else if v < 0x20 {
                    lo |= 1u32 << v;
                } else if v < 0x40 {
                    hi |= 1u32 << (v - 32);
                }
            }
        }
    }
    ((hi as u64) << 32) | lo as u64
});
