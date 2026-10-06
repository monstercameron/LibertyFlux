// original: 0x00bf7560 emit_init_2c
/// Initialise a tag-0x2c record: two setup calls, then store the payload.
///
/// Calls the tag setter with 0x2c, then the five-argument registrar with
/// (0x2c, shared flag word, `a1`, `a0`, 1); both are thiscall on `this`.
/// Stores `a2`/`a3`/`a5`/`a4` to this `+0x18/0x1c/0x20/0x24` (note the
/// swapped middle pair: `a5` lands at `+0x20`, `a4` at `+0x24`), the low
/// byte of `a6` to this `+0x14`, and the ready flag 2 to this `+2`.
/// Returns the registrar's answer with its low byte replaced by `a6`'s low
/// byte (the original reloads AL after the call).
///
/// Original: 0x00BF7560 (thiscall, seven stack words).
lf_checker_rt::export!(thiscall, rw_00bf7560(this: u32, a0: u32, a1: u32, a2: f32, a3: f32, a4: f32, a5: f32, a6: u32) -> u32 {
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
        const TAG: u32 = 0x2c;
        const FLAG_GLOB: u32 = 0x011735a4;
        const READY: u8 = 2;
        let flag: u32 = lf_checker_rt::global::<u32>(FLAG_GLOB).read();
        lf_checker_rt::callee_thiscall!(1, u32, this, TAG);
        let r = lf_checker_rt::callee_thiscall!(2, u32, this, TAG, flag, a1, a0, 1);
        wrf(this + 0x18, a2);
        wrf(this + 0x1c, a3);
        wrf(this + 0x20, a5);
        wrf(this + 0x24, a4);
        wr8(this + 0x14, a6 as u8);
        wr8(this + 2, READY);
        (r & 0xffffff00) | (a6 & 0xff)
    }
});
