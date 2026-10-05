// original: 0x00a056f0 NativeImpl_IS_OBJECT_IN_ANGLED_AREA_2D (native)
/// Test a handle against a 2D angled area.
///
/// The mode-0 form of the shared area tester: both vectors are 3 words
/// with the depth defaulted to -100.0 ([bx, by, -100.0] and [ax, ay,
/// -100.0]), followed by the two live scalar words. Cdecl, seven words.
lf_checker_rt::export!(cdecl, rw_00a056f0(handle: u32, ax: u32, ay: u32, bx: u32,
                                          by: u32, c1: u32, c2: u32) -> u32 {
    unsafe {
        const TEST: u32 = 0;
        const NEG100: u32 = 0xc2c8_0000;
        let corner = [bx, by, NEG100];
        let area = [ax, ay, NEG100];
        lf_checker_rt::callee_cdecl!(TEST, u32, 0u32, handle, area.as_ptr() as u32,
                                     corner.as_ptr() as u32, c1, c2)
    }
});
