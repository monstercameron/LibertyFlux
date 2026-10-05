// original: 0x00a056f0 NativeImpl_IS_OBJECT_IN_ANGLED_AREA_2D (native)
/// Test a handle against a 2D angled area (corner pair plus scalars).
///
/// Builds the corner vector [bx, by, 0, -100.0] and the area vector
/// [ax, ay] on the stack (the third corner word is stack garbage in the
/// original; the checker pins it to zero and so does this rewrite) and
/// asks the area tester with mode 0. The last handle word is pushed and
/// then overwritten before the call, so it is dead. Cdecl, seven words.
lf_checker_rt::export!(cdecl, rw_00a056f0(handle: u32, ax: u32, ay: u32, bx: u32,
                                          by: u32, c1: u32, _dead: u32) -> u32 {
    unsafe {
        const TEST: u32 = 0;
        const NEG100: u32 = 0xc2c8_0000;
        let corner = [bx, by, 0u32, NEG100];
        let area = [ax, ay];
        lf_checker_rt::callee_cdecl!(TEST, u32, 0u32, handle, area.as_ptr() as u32,
                                     corner.as_ptr() as u32, c1, NEG100)
    }
});
