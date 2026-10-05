// original: 0x00b970d0 NativeImpl_SET_GRAVITY_OFF

/// Applies or clears the gravity override selected by a flag byte.
///
/// With a nonzero low byte, zeroes a 12-byte scratch struct in its own
/// frame and passes its address to `FILL_CALLEE`; the filled struct is never
/// read afterwards. With a zero byte, calls `CLEAR_CALLEE` with no
/// arguments instead. No value is returned.
///
/// The scratch pointer differs legitimately between the two sides' frames,
/// so the contract skips that call argument and snapshot-verifies the
/// struct holds zeros at call time (see `narrowed`).
///
/// Original: 0x00B970D0 (cdecl, one stack word, no return value).
lf_checker_rt::export!(cdecl, rw_00b970d0(flag: u32) -> u32 {
    const FILL_CALLEE: u32 = 1;
    const CLEAR_CALLEE: u32 = 2;
    if flag & 0xFF != 0 {
        let mut scratch = [0u32; 3];
        let _: u32 = lf_checker_rt::callee_cdecl!(FILL_CALLEE, u32, scratch.as_mut_ptr() as u32);
    } else {
        let _: u32 = lf_checker_rt::callee_cdecl!(CLEAR_CALLEE, u32,);
    }
    0
});
