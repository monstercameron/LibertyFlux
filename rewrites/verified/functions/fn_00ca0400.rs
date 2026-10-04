// original: 0x00ca0400 float_max_store

/// Store a float sample and raise the recorded maximum.
///
/// `this` points to the IK parameter block, `v` is float bits. The sample is
/// always written to `+0x9c`; it is also written to `+0xb8` when it is
/// strictly greater than the current value at `+0x88` (an unordered NaN
/// comparison stores nothing, matching `comiss` + `jbe`). No value returned.
///
/// Original: 0x00ca0400 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00ca0400(this: u32, v: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    #[inline(always)]
    unsafe fn rdf(a: u32) -> f32 {
        unsafe { f32::from_bits(rd32(a)) }
    }
    #[inline(always)]
    unsafe fn wrf(a: u32, v: f32) {
        unsafe { wr32(a, v.to_bits()) }
    }
    unsafe {
        const CUR: u32 = 0x88;
        const SAMPLE: u32 = 0x9c;
        const MAXV: u32 = 0xb8;
        let sample = f32::from_bits(v);
        let cur = rdf(this + CUR);
        wrf(this + SAMPLE, sample);
        if sample > cur {
            wrf(this + MAXV, sample);
        }
        0
    }
});
