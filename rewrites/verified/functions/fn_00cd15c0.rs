// original: 0x00cd15c0 melee_id_find
/// Search the object's id table for a value, returning 1 if present else 0.
///
/// `this+0x60` holds the entry count and `this+0x64` an array of that many
/// 8-byte entries whose first dword is compared against `val`. The scan is
/// unsigned-bound (`jb`) and stops at the first match. Thiscall with one
/// stack argument; only the low byte of the result is meaningful (the rest
/// is entry-register or pointer leftovers), so the contract compares AL.
export!(thiscall, rw_00cd15c0(this: u32, val: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x60;
        const ITEMS_OFF: u32 = 0x64;
        const STRIDE: u32 = 8;
        let count = (this.wrapping_add(COUNT_OFF) as *const u32).read_unaligned();
        let mut i = 0u32;
        while i < count {
            let slot = (this
                .wrapping_add(ITEMS_OFF)
                .wrapping_add(i.wrapping_mul(STRIDE)) as *const u32)
                .read_unaligned();
            if val == slot {
                return 1;
            }
            i = i.wrapping_add(1);
        }
        0
    }
});
