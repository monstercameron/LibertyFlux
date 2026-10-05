// original: 0x00e6c770 veh_ratio_init_70 (proposed)

/// Recompute one vehicle tuning ratio from two live globals: `DST = SRC_NUMER / SRC_DENOM`.
///
/// All three operands are single floats in writable game data; the original is
/// three SSE instructions (`movss`, `divss`, `movss`) with no arguments and no
/// return value. Division by zero yields infinity and 0/0 the default quiet
/// NaN, exactly as the hardware instruction does; the rewrite issues the same
/// single division with the operands pinned in the original's order.
///
/// Original: 0x00e6c770 (cdecl, no arguments, no return channel).
lf_checker_rt::export!(cdecl, rw_00e6c770() -> u32 {
    unsafe {
        const SRC_NUMER: u32 = 0x01053ca4;
        const SRC_DENOM: u32 = 0x01053ca8;
        const DST_RATIO: u32 = 0x0171de58;
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        let a = f32::from_bits((lf_checker_rt::global::<u32>(SRC_NUMER)).read());
        let b = f32::from_bits((lf_checker_rt::global::<u32>(SRC_DENOM)).read());
        let q = div(a, b);
        (lf_checker_rt::global::<u32>(DST_RATIO)).write(q.to_bits());
        0
    }
});
