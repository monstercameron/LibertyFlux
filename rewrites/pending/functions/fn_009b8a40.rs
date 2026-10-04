// original: 0x009b8a40 NativeImpl_SET_CAM_POINT_DAMPING_PARAMS
/// Engine worker behind the `SET_CAM_POINT_DAMPING_PARAMS` native.
///
/// Allocates a 24-byte damping-params instruction block, stamps the class
/// word and a zero flag byte, then stores one integer field and three float
/// bit-patterns from the stack arguments. Returns null when allocation
/// fails.
export!(stdcall, rw_009b8a40(a0: u32, f1: u32, f2: u32, f3: u32) -> u32 {
    const CLASS: u32 = 0x00E94148;
    unsafe {
        let p = callee_cdecl!(0, u32, 0x18u32) as *mut u8;
        if p.is_null() {
            return 0;
        }
        *(p as *mut u32) = relocated(CLASS);
        *p.add(4) = 0;
        *(p.add(8) as *mut u32) = a0;
        *(p.add(0x0c) as *mut u32) = f1;
        *(p.add(0x10) as *mut u32) = f2;
        *(p.add(0x14) as *mut u32) = f3;
        p as u32
    }
});
