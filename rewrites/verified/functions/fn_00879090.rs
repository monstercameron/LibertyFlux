// original: 0x00879090 filter_slot_push

/// Push `value` into the filter's inline slot array.
///
/// `this` points to a filter object with a slot count at `+0x2C` and eight
/// inline slots starting at `+0x0C`. While the count is below 8 (SIGNED
/// comparison: a negative count still stores, below the array), `value` is
/// stored at `+0x0C + count * 4`, the count is bumped, and 1 is returned in
/// AL. A count of 8 or more (signed) returns AL 0 and stores nothing.
///
/// Original: 0x00879090 (thiscall, one stack argument; low byte of the
/// return only, the upper bytes keep the caller's value).
lf_checker_rt::export!(thiscall, rw_00879090(this: u32, value: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x2c;
        const SLOTS_OFF: u32 = 0x0c;
        const MAX_SLOTS: i32 = 8;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let count = rd32(this.wrapping_add(COUNT_OFF)) as i32;
        if count >= MAX_SLOTS {
            return 0;
        }
        wr32(this.wrapping_add(COUNT_OFF), (count.wrapping_add(1)) as u32);
        wr32(
            this.wrapping_add(SLOTS_OFF).wrapping_add((count as u32).wrapping_mul(4)),
            value,
        );
        1
    }
});
