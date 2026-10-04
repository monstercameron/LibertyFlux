// original: 0x00ca0430 float_min_store

/// Store a float sample and lower the recorded minimum.
///
/// Mirror of `float_max_store`: the sample `v` is always written to `+0xa0`
/// and additionally to `+0xb4` when the current value at `+0x88` is strictly
/// greater than the sample. NaN comparisons store nothing. No value returned.
///
/// Original: 0x00ca0430 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00ca0430(this: u32, v: u32) -> u32 {
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
        const SAMPLE: u32 = 0xa0;
        const MINV: u32 = 0xb4;
        let sample = f32::from_bits(v);
        let cur = rdf(this + CUR);
        wrf(this + SAMPLE, sample);
        if cur > sample {
            wrf(this + MINV, sample);
        }
        0
    }
});
