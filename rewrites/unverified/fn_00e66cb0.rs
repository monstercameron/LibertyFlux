// original: 0x00E66CB0 scaled_sqrt_store_00

/// Store `sqrt(d0) as f32 * m` into a global, from a global double pair.
///
/// Loads two `f64` values at `DBL`, takes the square root of each (only the
/// low lane's root is used; the high lane is computed for identical
/// floating-point behaviour), converts the low root to `f32`, multiplies it by
/// the global float at `F`, and writes the product bits to `DST`. All three
/// operations are the original's hardware instructions (`sqrtsd`, `cvtsd2ss`,
/// `mulss`) with operands pinned through `black_box`, so edge values
/// (negative inputs, infinities, NaNs, subnormals) match bit for bit.
///
/// Original: 0x00E66CB0 (cdecl, no arguments, no outgoing calls, no return value).
lf_checker_rt::export!(cdecl, rw_00e66cb0() -> u32 {
    unsafe {
        const DBL: u32 = 0x00E98890;
        const F: u32 = 0x00FE8DB0;
        const DST: u32 = 0x012B6090;
        let d0 = f64::from_bits((lf_checker_rt::global::<u64>(DBL)).read_unaligned());
        let d1 = f64::from_bits((lf_checker_rt::global::<u64>(DBL + 8)).read_unaligned());
        let s0 = core::hint::black_box(d0).sqrt();
        let s1 = core::hint::black_box(d1).sqrt();
        core::hint::black_box(s1);
        let m = f32::from_bits((lf_checker_rt::global::<u32>(F)).read_unaligned());
        let p = core::hint::black_box(s0 as f32) * core::hint::black_box(m);
        (lf_checker_rt::global::<u32>(DST)).write_unaligned(p.to_bits());
    }
    0
});
