// original: 0x00bf75c0 emit_init_30
/// Initialise a tag-0x30 record: three setup calls, then fold a flag bit.
///
/// Calls the tag setter with 0x30, the five-argument registrar with (0x30,
/// shared flag word, `a1`, `a0`, 1), and the applier with `a3`; all are
/// thiscall on `this`, and all answers are ignored. Then toggles bit 0 of
/// this `+0x24` when it differs from bit 0 of the low byte of `a2` (all
/// unsigned byte logic), writes the ready flag 1 to this `+2`, derives
/// this `+0x14` as bit 0 plus 6, and returns bit 0 of the folded byte.
///
/// Original: 0x00BF75C0 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00bf75c0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const TAG: u32 = 0x30;
        const FLAG_GLOB: u32 = 0x011735a4;
        const FLAG_BYTE: u32 = 0x24;
        const READY: u8 = 1;
        const KIND_BASE: u8 = 6;
        let flag: u32 = lf_checker_rt::global::<u32>(FLAG_GLOB).read();
        lf_checker_rt::callee_thiscall!(1, u32, this, TAG);
        lf_checker_rt::callee_thiscall!(2, u32, this, TAG, flag, a1, a0, 1);
        lf_checker_rt::callee_thiscall!(3, u32, this, a3);
        let cur = rd8(this + FLAG_BYTE);
        let bit = (cur ^ (a2 as u8)) & 1;
        wr8(this + 2, READY);
        wr8(this + FLAG_BYTE, cur ^ bit);
        let new = rd8(this + FLAG_BYTE);
        wr8(this + 0x14, (new & 1).wrapping_add(KIND_BASE));
        (new & 1) as u32
    }
});
