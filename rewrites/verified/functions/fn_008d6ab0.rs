// original: 0x008d6ab0 zero_vec_guard
/// Replace an exact (0, 0) float pair with a tiny epsilon.
///
/// If both words read as ordered-equal to zero (either sign of zero; NaN
/// takes the early-out path), both are overwritten with 1e-4; otherwise the
/// pair is left untouched. Returns nothing (EAX preserved).
export!(cdecl, rw_008d6ab0(pair: *mut f32) -> u32 {
    const EPSILON: f32 = f32::from_bits(0x38D1_B717);
    unsafe {
        let x = *pair;
        let y = *pair.add(1);
        if x == 0.0 && y == 0.0 {
            *pair = EPSILON;
            *pair.add(1) = EPSILON;
        }
        // EAX is preserved by the original; return value is not compared,
        // but keep the signature honest: read nothing, return EntryEAX is
        // impossible, so return 0 (ret channel is `none`).
        0
    }
});
