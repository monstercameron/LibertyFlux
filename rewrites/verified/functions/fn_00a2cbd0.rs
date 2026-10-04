// original: 0x00a2cbd0 ped_timer_advance

/// Advance the ped's timer toward its clamp.
/// Computes `G_RATE * G_STEP + [this+0xED0]` in the original's order and
/// stores it back. When the result is above the global limit `G_LIM` the
/// timer overflows: `+0xED4` is set to the 10.0 bit pattern and `+0xED0`
/// is reset to +0.0. NaN results count as not above and skip the clamp.
/// Original: 0x00a2cbd0 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00a2cbd0(this: u32) -> u32 {
    unsafe {
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe fn rdf(a: u32) -> f32 {
        unsafe { f32::from_bits(rd32(a)) }
    }
    unsafe fn wrf(a: u32, v: f32) {
        unsafe { wr32(a, v.to_bits()) }
    }
    unsafe fn gbit(file_va: u32) -> f32 {
        unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(file_va))) }
    }
    fn fmul(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
    fn fadd(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) + core::hint::black_box(b)
    }
        const VAL: u32 = 0xed0;
        const OVER: u32 = 0xed4;
        const G_RATE: u32 = 0x11735bc;
        const G_STEP: u32 = 0x00fe8914;
        const G_LIM: u32 = 0x00fe8b38;
        const OVERFLOW_BITS: u32 = 0x41200000;
        let v = fadd(fmul(gbit(G_RATE), gbit(G_STEP)), rdf(this.wrapping_add(VAL)));
        wrf(this.wrapping_add(VAL), v);
        if v > gbit(G_LIM) {
            wr32(this.wrapping_add(OVER), OVERFLOW_BITS);
            wrf(this.wrapping_add(VAL), 0.0);
        }
        0
    }
});
