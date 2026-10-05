// original: 0x00a05a90 NativeImpl_LOCATE_OBJECT_2D (native)
/// Locate a handle in 2D: the locate tester with mode 0.
///
/// Vectors [bx, by, 0.0] and [ax, ay, -100.0] plus the live trailing
/// scalar. Cdecl, six words.
lf_checker_rt::export!(cdecl, rw_00a05a90(handle: u32, ax: u32, ay: u32, bx: u32,
                                          by: u32, c: u32) -> u32 {
    unsafe {
        const TEST: u32 = 0;
        const NEG100: u32 = 0xc2c8_0000;
        let va = [bx, by, 0u32];
        let vb = [ax, ay, NEG100];
        lf_checker_rt::callee_cdecl!(TEST, u32, 0u32, handle, vb.as_ptr() as u32,
                                     va.as_ptr() as u32, c)
    }
});
