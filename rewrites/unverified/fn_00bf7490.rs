// original: 0x00bf7490 emit_init_37
/// Initialise a tag-0x37 record: three setup calls, then store the payload.
///
/// Calls the tag setter with 0x37, then the five-argument registrar with
/// (0x37, shared flag word, `a1`, `a0`, 1), then the encoder with `a1`; all
/// are thiscall on `this`. Stores `a3/a4/a5` to this `+0x28/0x2c/0x30`,
/// writes 0x0a to this `+0x14` and the ready flag 1 to this `+2`. Returns
/// the encoder call's answer. `a2` is not read. No value is compared.
///
/// Original: 0x00BF7490 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_00bf7490(this: u32, a0: u32, a1: u32, _a2: u32, a3: f32, a4: f32, a5: f32) -> u32 {
    unsafe {
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
        const TAG: u32 = 0x37;
        const FLAG_GLOB: u32 = 0x011735a4;
        const READY: u8 = 1;
        const KIND: u8 = 0x0a;
        let flag: u32 = lf_checker_rt::global::<u32>(FLAG_GLOB).read();
        lf_checker_rt::callee_thiscall!(1, u32, this, TAG);
        lf_checker_rt::callee_thiscall!(2, u32, this, TAG, flag, a1, a0, 1);
        let r = lf_checker_rt::callee_thiscall!(3, u32, this, a1);
        wrf(this + 0x28, a3);
        wrf(this + 0x2c, a4);
        wrf(this + 0x30, a5);
        wr8(this + 0x14, KIND);
        wr8(this + 2, READY);
        r
    }
});
