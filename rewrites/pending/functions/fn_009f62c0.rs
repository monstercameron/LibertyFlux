// original: 0x009f62c0 code_to_float
/// Small integer-to-float code map: 0 -> 0.0, 7 -> 600.0, else 1.0.
export!(cdecl, rw_009f62c0(code: u32) -> f32 {
    match code {
        0 => 0.0,
        7 => 600.0,
        _ => 1.0,
    }
});
