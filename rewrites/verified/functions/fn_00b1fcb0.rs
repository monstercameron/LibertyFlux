// original: 0x00b1fcb0 scale_vec_by_measured (proposed)

/// Scales a three-float vector by a freshly measured factor.
///
/// Cdecl of two stack words: a tag and a pointer to three floats. Runs
/// the measurer (thiscall on the shared block at 0x1633810 with both
/// words), reads the factor the estimator left in ST0, and multiplies
/// each component by it in `component * factor` order. EAX on return is
/// the estimator's leftover, which the harness cannot reproduce, so no
/// return channel is compared.
lf_checker_rt::export!(cdecl, rw_00b1fcb0(tag: u32, vec: u32) -> () {
    unsafe {
        const SHARED: u32 = 0x01633810;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(SHARED), tag, vec);
        let k: f32 = lf_checker_rt::callee_cdecl!(2, f32,);
        for off in [0u32, 4, 8] {
            let p = (vec + off) as *mut f32;
            p.write_unaligned(mul(p.read_unaligned(), k));
        }
    }
});
