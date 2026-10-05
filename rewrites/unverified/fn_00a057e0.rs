// original: 0x00a057e0 NativeImpl_IS_OBJECT_IN_AREA_2D (native)
/// Test a handle against a 2D area box.
///
/// The mode-0 form of the box tester: the corner vector [bx, by, -100.0],
/// the area vector [ax, ay, -100.0], and the live trailing scalar. Cdecl,
/// six words.
lf_checker_rt::export!(cdecl, rw_00a057e0(handle: u32, ax: u32, ay: u32, bx: u32,
                                          by: u32, c: u32) -> u32 {
    unsafe {
        const TEST: u32 = 0;
        const NEG100: u32 = 0xc2c8_0000;
        let va = [bx, by, NEG100];
        let vb = [ax, ay, NEG100];
        lf_checker_rt::callee_cdecl!(TEST, u32, 0u32, handle, vb.as_ptr() as u32,
                                     va.as_ptr() as u32, c)
    }
});
