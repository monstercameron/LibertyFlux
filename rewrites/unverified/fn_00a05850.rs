// original: 0x00a05850 NativeImpl_IS_OBJECT_IN_AREA_3D (native)
/// Test a handle against a 3D area box.
///
/// The mode-1 form: two 3-vectors [ax, ay, az] and [bx, by, bz] plus the
/// trailing scalar word. Cdecl, eight words.
lf_checker_rt::export!(cdecl, rw_00a05850(handle: u32, ax: u32, ay: u32, az: u32,
                                          bx: u32, by: u32, bz: u32, c: u32) -> u32 {
    unsafe {
        const TEST: u32 = 0;
        let va = [bx, by, bz];
        let vb = [ax, ay, az];
        lf_checker_rt::callee_cdecl!(TEST, u32, 1u32, handle, vb.as_ptr() as u32,
                                     va.as_ptr() as u32, c)
    }
});
