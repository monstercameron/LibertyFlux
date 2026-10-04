// original: 0x00a72e40 CTaskComplexPlayerGun::vf5 (symbols)
/// Runs the sub-task's step hook, resets the task fields, notifies twice.
///
/// `thiscall`: object in ECX, three stack words, callee pops 12. The
/// sub-task hook behaves as in the sibling step functions (skipped when
/// bit 0 of the flag byte at `sub+0xc` is set, else virtual slot `+0x14`
/// with the three words; a zero answer returns 0; a hook that ran latches
/// bit 1). Then the reset call (callee 2, thiscall on `this+0x14`) runs,
/// the fields at `+0x24`/`+0x28`/`+0x2c` become -1/0/0, the notify call
/// (callee 3, thiscall on the object) runs, and the result is 1.
lf_checker_rt::export!(thiscall, rw_00a72e40(this: u32, a: u32, b: u32, c: u32) -> u8 {
    unsafe {
        const SUB_OFF: u32 = 0x8;
        const FLAG_OFF: u32 = 0xc;
        const HOOK_SLOT: u32 = 0x14;
        const RESET_OFF: u32 = 0x14;
        const F_A: u32 = 0x24;
        const F_B: u32 = 0x28;
        const F_C: u32 = 0x2c;
        let sub = ((this + SUB_OFF) as *const u32).read_unaligned();
        if ((sub + FLAG_OFF) as *const u8).read() & 1 == 0 {
            let vt = (sub as *const u32).read_unaligned();
            let hook: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(
                    (((vt + HOOK_SLOT) as *const u32).read_unaligned()) as usize,
                );
            if hook(sub, a, b, c) as u8 == 0 {
                return 0;
            }
            let w = ((sub + FLAG_OFF) as *const u32).read_unaligned();
            ((sub + FLAG_OFF) as *mut u32).write_unaligned(w | 2);
        }
        lf_checker_rt::callee_thiscall!(2, u32, this.wrapping_add(RESET_OFF));
        ((this + F_A) as *mut u32).write_unaligned(0xffff_ffff);
        ((this + F_B) as *mut u8).write(0);
        ((this + F_C) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(3, u32, this);
        1
    }
});
