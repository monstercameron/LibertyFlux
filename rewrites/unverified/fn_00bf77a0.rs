// original: 0x00bf77a0 emit_init_39
/// Initialise a tag-0x39 record: register, apply once, then store a float.
///
/// Calls the five-argument registrar as thiscall on `this` with (0x39,
/// shared flag word, `a1`, `a0`, 0), then one applier call with `a2`.
/// Stores `a3` to this `+0x1c` and the ready flag 1 to this `+2`. Returns
/// the applier's answer.
///
/// Original: 0x00BF77A0 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00bf77a0(this: u32, a0: u32, a1: u32, a2: u32, a3: f32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const TAG: u32 = 0x39;
        const FLAG_GLOB: u32 = 0x011735a4;
        const READY: u8 = 1;
        let flag: u32 = lf_checker_rt::global::<u32>(FLAG_GLOB).read();
        lf_checker_rt::callee_thiscall!(1, u32, this, TAG, flag, a1, a0, 0);
        let r = lf_checker_rt::callee_thiscall!(2, u32, this, a2);
        wrf(this + 0x1c, a3);
        wr8(this + 2, READY);
        r
    }
});
