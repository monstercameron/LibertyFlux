// original: 0x00bf7610 emit_init_31
/// Initialise a tag-0x31 record: register, apply twice, then store one byte.
///
/// Calls the five-argument registrar as thiscall on `this` with (0x31,
/// shared flag word, `a1`, `a0`, 0), then two applier calls with `a3` and
/// `a4`. Stores the low byte of `a2` to this `+0x1c` and the ready flag 1
/// to this `+2`. Returns the second applier's answer.
///
/// Original: 0x00BF7610 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_00bf7610(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const TAG: u32 = 0x31;
        const FLAG_GLOB: u32 = 0x011735a4;
        const READY: u8 = 1;
        let flag: u32 = lf_checker_rt::global::<u32>(FLAG_GLOB).read();
        lf_checker_rt::callee_thiscall!(1, u32, this, TAG, flag, a1, a0, 0);
        lf_checker_rt::callee_thiscall!(2, u32, this, a3);
        let r = lf_checker_rt::callee_thiscall!(3, u32, this, a4);
        wr8(this + 0x1c, a2 as u8);
        wr8(this + 2, READY);
        r
    }
});
