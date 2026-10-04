// original: 0x009FF300 frag_push_default_vector (proposed)

/// Submit the constant vector (0, 0, -9.61) to the vector sink callee.
///
/// Builds three floats on the frame (0.0, 0.0, 0xC11CCCCD) and passes a
/// pointer to them to the callee. Returns whatever the callee returned.
///
/// Original: 0x009FF300 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_009FF300() -> u32 {
    unsafe {
        const Z: f32 = f32::from_bits(0xC11C_CCCD);
        let v = [0.0f32, 0.0f32, Z];
        lf_checker_rt::callee_cdecl!(1, u32, v.as_ptr() as u32)
    }
});
