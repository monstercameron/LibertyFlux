// original: 0x00c245f0 cam_range_normalize (proposed)
/// Normalize `v` into [0, 1] against row `index` of the parameter table:
/// returns 0 when the row's low bound is strictly above `v`, 1 when `v` is
/// strictly above the row's high bound, else `(v - lo) / (hi - lo)`. Each
/// `comiss` + `jbe` continues past the early return on unordered inputs too,
/// so the guards are the strict ordered relations. The division is one SSE
/// divide; a zero span yields infinity or NaN exactly as the original's.
///
/// Original: 0x00c245f0 (thiscall, two stack words; float result).
lf_checker_rt::export!(thiscall, rw_00c245f0(obj: u32, fv: u32, index: u32) -> f32 {
    unsafe {
        const LO_OFF: u32 = 0x198;
        const HI_OFF: u32 = 0x1c0;
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let v = f32::from_bits(fv);
        let row = obj.wrapping_add(index.wrapping_mul(4));
        let lo = (row.wrapping_add(LO_OFF) as *const f32).read_unaligned();
        let hi = (row.wrapping_add(HI_OFF) as *const f32).read_unaligned();
        if lo > v {
            return 0.0;
        }
        if v > hi {
            return 1.0;
        }
        let num = sub(v, lo);
        let den = sub(hi, lo);
        core::hint::black_box(num) / core::hint::black_box(den)
    }
});
