// original: 0x00bf7520 emit_init_2d
/// Initialise a tag-0x2d record: register, then store the payload.
///
/// Calls the five-argument registrar as thiscall on `this` with (0x2d,
/// shared flag word, `a1`, `a0`, 0). Stores `a2`/`a3` to this `+0x10/0x14`,
/// the low byte of `a4` to this `+0x18`, and the ready flag 2 to this `+2`.
/// Returns the registrar's answer with its low byte replaced by `a4`'s low
/// byte (the original reloads AL after the call). Only that byte of `a4`
/// is observed.
///
/// Original: 0x00BF7520 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_00bf7520(this: u32, a0: u32, a1: u32, a2: f32, a3: f32, a4: u32) -> u32 {
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
        const TAG: u32 = 0x2d;
        const FLAG_GLOB: u32 = 0x011735a4;
        const READY: u8 = 2;
        let flag: u32 = lf_checker_rt::global::<u32>(FLAG_GLOB).read();
        let r = lf_checker_rt::callee_thiscall!(1, u32, this, TAG, flag, a1, a0, 0);
        wrf(this + 0x10, a2);
        wrf(this + 0x14, a3);
        wr8(this + 0x18, a4 as u8);
        wr8(this + 2, READY);
        (r & 0xffffff00) | (a4 & 0xff)
    }
});
