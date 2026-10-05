// original: 0x00a53240 vehicle_add_part (proposed)

/// Add a part at index `idx`, allocating one when asked.
///
/// An index of -1 takes the counter at +0x1950 and bumps it; any other
/// index is used as given. The part callee then runs with the part address
/// (+0x90 plus index times 0xb0) in ecx and the three leading arguments,
/// and the index's presence byte (+0x1960) is set. Thiscall, four stack
/// words, one callee, no result.
lf_checker_rt::export!(thiscall, rw_00a53240(this: u32, a0: u32, a1: u32, a2: u32, idx: u32) -> u32 {
    unsafe {
        const PARTS_BASE: u32 = 0x90;
        const PART_STRIDE: u32 = 0xb0;
        const COUNTER: u32 = 0x1950;
        const PRESENT: u32 = 0x1960;
        const AUTO: u32 = 0xffff_ffff;
        const ADD: u32 = 1;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let i: u32;
        if idx == AUTO {
            i = rd32(this.wrapping_add(COUNTER));
            (this.wrapping_add(COUNTER) as *mut u32).write_unaligned(i.wrapping_add(1));
        } else {
            i = idx;
        }
        let part = this
            .wrapping_add(PARTS_BASE)
            .wrapping_add(i.wrapping_mul(PART_STRIDE));
        lf_checker_rt::callee_thiscall!(ADD, u32, part, a0, a1, a2);
        (i.wrapping_add(this).wrapping_add(PRESENT) as *mut u8).write(1);
        0
    }
});
