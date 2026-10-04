// original: 0x00a72f30 CTaskComplexPlayerInCover::vf5 (symbols)
/// Runs the sub-task's step hook, then the base-class step, returning 1
/// unless the hook vetoes.
///
/// `thiscall`: object in ECX, three stack words, callee pops 12. Loads
/// the sub-task at `this+8`; when its flag byte at `+0xc` has bit 0 set
/// the hook is skipped, otherwise the hook (virtual slot at `+0x14`,
/// thiscall on the sub-task with the three words) runs and a zero answer
/// returns 0 at once. A hook that ran latches bit 1 of the flag word.
/// Then the base step (callee 2, thiscall on the object with the first
/// word) runs and the result is 1.
lf_checker_rt::export!(thiscall, rw_00a72f30(this: u32, a: u32, b: u32, c: u32) -> u8 {
    unsafe {
        const SUB_OFF: u32 = 0x8;
        const FLAG_OFF: u32 = 0xc;
        const HOOK_SLOT: u32 = 0x14;
        const SKIP_BIT: u8 = 1;
        const DONE_BIT: u32 = 2;
        let sub = ((this + SUB_OFF) as *const u32).read_unaligned();
        if ((sub + FLAG_OFF) as *const u8).read() & SKIP_BIT == 0 {
            let vt = (sub as *const u32).read_unaligned();
            let hook: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(
                    (((vt + HOOK_SLOT) as *const u32).read_unaligned()) as usize,
                );
            if hook(sub, a, b, c) as u8 == 0 {
                return 0;
            }
            let w = ((sub + FLAG_OFF) as *const u32).read_unaligned();
            ((sub + FLAG_OFF) as *mut u32).write_unaligned(w | DONE_BIT);
        }
        lf_checker_rt::callee_thiscall!(2, u32, this, a);
        1
    }
});
