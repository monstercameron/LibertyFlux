// original: 0x00974a90 audio_dual_param_product (proposed)

/// Multiply two parameter fetches, on ST0.
///
/// A zero argument gives +0.0. Otherwise two 2-bit fields of the argument
/// (bits 2..3, then bits 0..1) each select an index converted exactly as in
/// the single fetch (double plus the read-only table base, narrowed to
/// float) and passed to the parameter callee with its fixed object address.
/// Returns the second result times the first, in that multiply order.
/// Original: 0x00974A90 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00974a90(arg: u32) -> f32 {
    unsafe {
        const OBJ1: u32 = 0x121FA10;
        const OBJ2: u32 = 0x121F9D4;
        const TABLE0: f64 = f64::from_bits(0x0000000000000000);
        const PARAM1: u32 = 1;
        const PARAM2: u32 = 2;
        if arg == 0 {
            return 0.0;
        }
        let i1 = (arg >> 2) & 3;
        let a1 = (core::hint::black_box(i1 as f64) + core::hint::black_box(TABLE0)) as f32;
        let r1: f32 = lf_checker_rt::callee_thiscall!(
            PARAM1,
            f32,
            lf_checker_rt::relocated(OBJ1),
            a1.to_bits()
        );
        let i2 = arg & 3;
        let a2 = (core::hint::black_box(i2 as f64) + core::hint::black_box(TABLE0)) as f32;
        let r2: f32 = lf_checker_rt::callee_thiscall!(
            PARAM2,
            f32,
            lf_checker_rt::relocated(OBJ2),
            a2.to_bits()
        );
        core::hint::black_box(r2) * core::hint::black_box(r1)
    }
});
