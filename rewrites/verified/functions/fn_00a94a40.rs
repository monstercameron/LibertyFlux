// original: 0x00a94a40 stream_slot_in_range (proposed)

/// Whether slot `idx` is a usable stream slot (1) or not (0).
///
/// The table object holds a capacity at `+0x04` and the entry array at
/// `+0x00`. The slot is usable when the capacity is non-negative, the array
/// is non-null, the index is non-negative and not the `0xffff` sentinel,
/// and the index is below `capacity + 6` (signed, wrapping add). Only `al`
/// carries the result, so the comparison covers `al` only. Pure leaf.
///
/// Original: thiscall, one stack argument.
lf_checker_rt::export!(thiscall, rw_00a94a40(this: u32, idx: u32) -> u8 {
    unsafe {
        const TABLE_CAP: u32 = 0x04;
        const TABLE_BASE: u32 = 0x00;
        const NO_SLOT: u32 = 0xffff;
        const SLACK: i32 = 6;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let cap = rd32(this.wrapping_add(TABLE_CAP)) as i32;
        if cap < 0 {
            return 0;
        }
        if rd32(this.wrapping_add(TABLE_BASE)) == 0 {
            return 0;
        }
        let i = idx as i32;
        if idx == NO_SLOT || i < 0 {
            return 0;
        }
        if i >= cap.wrapping_add(SLACK) {
            return 0;
        }
        1
    }
});
