// original: 0x00974a50 audio_indexed_param_fetch (proposed)

/// Fetch one parameter through the audio parameter object, on ST0.
///
/// A zero argument gives +0.0. Otherwise bits 0..1 of the argument select a
/// small index; the index as a double plus the read-only table base (the
/// table index, index >> 31, is always 0 for a 2-bit index) is narrowed to
/// float and passed to the parameter callee with the fixed object address.
/// Returns the callee's float result.
/// Original: 0x00974A50 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00974a50(arg: u32) -> f32 {
    unsafe {
        const OBJ: u32 = 0x121FA60;
        const TABLE0: f64 = f64::from_bits(0x0000000000000000);
        const PARAM: u32 = 1;
        if arg == 0 {
            return 0.0;
        }
        let idx = arg & 3;
        let tab = (idx >> 31) as usize;
        debug_assert!(tab == 0);
        let wide = core::hint::black_box(idx as f64) + core::hint::black_box(TABLE0);
        let f = wide as f32;
        lf_checker_rt::callee_thiscall!(
            PARAM,
            f32,
            lf_checker_rt::relocated(OBJ),
            f.to_bits()
        )
    }
});
