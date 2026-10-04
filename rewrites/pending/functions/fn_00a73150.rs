// original: 0x00a73150 CTaskComplexPlayerPlaceCarBomb::vf5 (symbols)
/// Conditionally resets, arms the task, then runs the sub-task's hook
/// with a fixed middle argument.
///
/// `thiscall`: object in ECX, three stack words, callee pops 12. When bit
/// 0 of the byte at `this+0x26` is set, the reset call runs first
/// (callee 1, thiscall on `this+0x1c` with -1). The byte at `this+0x18`
/// becomes 1. The sub-task hook (virtual slot `+0x14`, thiscall) is
/// skipped when bit 0 of `sub+0xc` is set, otherwise it runs with the
/// first word, the constant 2 and the third word; a zero answer returns 0
/// and a hook that ran latches bit 1. Otherwise the result is 1.
lf_checker_rt::export!(thiscall, rw_00a73150(this: u32, a: u32, _b: u32, c: u32) -> u8 {
    unsafe {
        const COND_OFF: u32 = 0x26;
        const RESET_OFF: u32 = 0x1c;
        const ARM_OFF: u32 = 0x18;
        const SUB_OFF: u32 = 0x8;
        const FLAG_OFF: u32 = 0xc;
        const HOOK_SLOT: u32 = 0x14;
        const FIXED_MID: u32 = 2;
        if ((this + COND_OFF) as *const u8).read() & 1 != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, this.wrapping_add(RESET_OFF), 0xffff_ffff);
        }
        ((this + ARM_OFF) as *mut u8).write(1);
        let sub = ((this + SUB_OFF) as *const u32).read_unaligned();
        if ((sub + FLAG_OFF) as *const u8).read() & 1 == 0 {
            let vt = (sub as *const u32).read_unaligned();
            let hook: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(
                    (((vt + HOOK_SLOT) as *const u32).read_unaligned()) as usize,
                );
            if hook(sub, a, FIXED_MID, c) as u8 == 0 {
                return 0;
            }
            let w = ((sub + FLAG_OFF) as *const u32).read_unaligned();
            ((sub + FLAG_OFF) as *mut u32).write_unaligned(w | 2);
        }
        1
    }
});
