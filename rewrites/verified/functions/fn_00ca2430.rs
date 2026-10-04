// original: 0x00ca2430 slot_shift_push

/// Push a value into the event-source slots, shifting non-zero entries up.
///
/// Each non-zero slot in `+0x15c .. +0x168` is copied one slot up (highest
/// first); zero slots neither move nor overwrite their target. The new value
/// is stored in the lowest slot. Returns the stored value (the original
/// leaves it in eax).
///
/// Original: 0x00ca2430 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00ca2430(this: u32, val: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe {
        const SLOTS: u32 = 0x15c;
        let hi = rd32(this + SLOTS + 12);
        if hi != 0 {
            wr32(this + SLOTS + 16, hi);
        }
        let m2 = rd32(this + SLOTS + 8);
        if m2 != 0 {
            wr32(this + SLOTS + 12, m2);
        }
        let m1 = rd32(this + SLOTS + 4);
        if m1 != 0 {
            wr32(this + SLOTS + 8, m1);
        }
        let lo = rd32(this + SLOTS);
        if lo != 0 {
            wr32(this + SLOTS + 4, lo);
        }
        wr32(this + SLOTS, val);
        val
    }
});
