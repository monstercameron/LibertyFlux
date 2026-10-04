// original: 0x00a2b530 ped_companion_ready

/// Report whether the ped's companion slot holds a live companion.
/// Returns nonzero when the pointer at `+0x30` is non-null, the count at
/// `+0x38` is positive, and the readiness check (callee 1, thiscall on the
/// slot with no stack arguments) answers nonzero; zero otherwise.
/// Original: 0x00a2b530 (thiscall, no stack arguments, byte result).
lf_checker_rt::export!(thiscall, rw_00a2b530(this: u32) -> u8 {
    unsafe {
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
        const SLOT: u32 = 0x30;
        const COUNT: u32 = 0x38;
        const READY: u32 = 1;
        let sub = rd32(this.wrapping_add(SLOT));
        if sub == 0 {
            return 0;
        }
        if (rd32(this.wrapping_add(COUNT)) as i32) <= 0 {
            return 0;
        }
        if lf_checker_rt::callee_thiscall!(READY, u8, sub) == 0 {
            0
        } else {
            1
        }
    }
});
