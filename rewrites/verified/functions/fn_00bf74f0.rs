// original: 0x00bf74f0 emit_init_38
/// Initialise a tag-0x38 record: register, then raise the ready flag.
///
/// Calls the five-argument registrar as thiscall on `this` with (0x38,
/// shared flag word, `a1`, `a0`, 0) and writes the ready flag 1 to this
/// `+2`. Returns the registrar's answer. The smallest of the cluster.
///
/// Original: 0x00BF74F0 (thiscall, two stack words: a0, a1).
lf_checker_rt::export!(thiscall, rw_00bf74f0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const TAG: u32 = 0x38;
        const FLAG_GLOB: u32 = 0x011735a4;
        const READY: u8 = 1;
        let flag: u32 = lf_checker_rt::global::<u32>(FLAG_GLOB).read();
        let r = lf_checker_rt::callee_thiscall!(1, u32, this, TAG, flag, a1, a0, 0);
        wr8(this + 2, READY);
        r
    }
});
