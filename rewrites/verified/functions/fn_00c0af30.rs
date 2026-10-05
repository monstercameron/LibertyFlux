// original: 0x00c0af30 stream_blend_factor (proposed)

/// Blend the streaming factor from the global level, gated by two checks.
///
/// Reads the level at `LEVEL`, takes its square root, scales it by `SCALE`
/// and subtracts that from `BASE`, all single-precision in that order (each
/// operand pinned so the compiler keeps the original's instruction order).
/// The mix checker (callee 1) receives the result bits; when it answers
/// nonzero the refresher (callee 2) runs. The override checker (callee 3)
/// then runs, and when it answers nonzero the result is replaced by 1.
/// Returns the result on the x87 stack.
///
/// Original: 0x00c0af30 (no arguments, no object; all callees cdecl).
lf_checker_rt::export!(cdecl, rw_00c0af30() -> f32 {
    unsafe {
        const LEVEL: u32 = 0x12ddeac;
        const BASE: u32 = 0xfe88e8;
        const SCALE: u32 = 0xfe8898;
        const MIX: u32 = 1;
        const REFRESH: u32 = 2;
        const OVERRIDE: u32 = 3;
        let level = (lf_checker_rt::relocated(LEVEL) as *const f32).read_unaligned();
        let base = (lf_checker_rt::relocated(BASE) as *const f32).read_unaligned();
        let scale = (lf_checker_rt::relocated(SCALE) as *const f32).read_unaligned();
        let rooted = core::hint::black_box(level).sqrt();
        let scaled = core::hint::black_box(rooted) * core::hint::black_box(scale);
        let mut out = core::hint::black_box(base) - core::hint::black_box(scaled);
        let m: u32 = lf_checker_rt::callee_cdecl!(MIX, u32, out.to_bits());
        if m & 0xff != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(REFRESH, u32,);
        }
        let o: u32 = lf_checker_rt::callee_cdecl!(OVERRIDE, u32,);
        if o & 0xff != 0 {
            out = 1.0;
        }
        out
    }
});
