// original: 0x00a057e0 NativeImpl_IS_OBJECT_IN_AREA_2D (native)
/// Test a handle against a 2D area box.
///
/// Passes the mode-0 form to the box tester: the 2-vector [bx, by] and the
/// 3-vector [ax, ay, -100.0], with -100.0 as the trailing scalar. The last
/// handle word is pushed and overwritten before the call, so it is dead.
/// Cdecl, six words.
lf_checker_rt::export!(cdecl, rw_00a057e0(handle: u32, ax: u32, ay: u32, bx: u32,
                                          by: u32, _dead: u32) -> u32 {
    unsafe {
        const TEST: u32 = 0;
        const NEG100: u32 = 0xc2c8_0000;
        let va = [bx, by];
        let vb = [ax, ay, NEG100];
        lf_checker_rt::callee_cdecl!(TEST, u32, 0u32, handle, vb.as_ptr() as u32,
                                     va.as_ptr() as u32, NEG100)
    }
});
