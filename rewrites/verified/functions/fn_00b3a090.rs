// original: 0x00B3A090 index_sum_clamp

/// Clamp the sum of two engine-global counters against a sampled limit.
///
/// `sum = G_A + G_B` (wrapping). Takes one sample `v1` from callee 1 (a
/// zero-argument sampler); when `sum < v1` (signed) the sum stands, otherwise
/// a second sample is taken and used. Returns the result minus `G_C`.
/// Cdecl, no stack arguments, returns in eax.
///
/// Original: 0x00B3A090.

lf_checker_rt::export!(cdecl, rw_00B3A090() -> u32 {
    unsafe {
        const G_A: u32 = 0x0169E408;
        const G_B: u32 = 0x0169E40C;
        const G_C: u32 = 0x016624C0;
        const SAMPLER: u32 = 1;
        let a = (lf_checker_rt::global::<u32>(G_A) as *const u32).read_unaligned();
        let b = (lf_checker_rt::global::<u32>(G_B) as *const u32).read_unaligned();
        let sum = a.wrapping_add(b);
        let v1: u32 = lf_checker_rt::callee_cdecl!(SAMPLER, u32,);
        let r = if (sum as i32) < (v1 as i32) {
            sum
        } else {
            lf_checker_rt::callee_cdecl!(SAMPLER, u32,)
        };
        let c = (lf_checker_rt::global::<u32>(G_C) as *const u32).read_unaligned();
        r.wrapping_sub(c)
    }
});
