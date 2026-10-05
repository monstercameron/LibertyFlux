// original: 0x00a05760 NativeImpl_IS_OBJECT_IN_ANGLED_AREA_3D (native)
/// Test a handle against a 3D angled area.
///
/// Stack-builds the 3-vectors [ax, ay, az] and [bx, by, bz] and asks the
/// area tester with mode 1, passing the two scalar words through. Cdecl,
/// nine words.
lf_checker_rt::export!(cdecl, rw_00a05760(handle: u32, ax: u32, ay: u32, az: u32,
                                          bx: u32, by: u32, bz: u32, c1: u32, c2: u32) -> u32 {
    unsafe {
        const TEST: u32 = 0;
        let va = [bx, by, bz];
        let vb = [ax, ay, az];
        lf_checker_rt::callee_cdecl!(TEST, u32, 1u32, handle, vb.as_ptr() as u32,
                                     va.as_ptr() as u32, c1, c2)
    }
});
