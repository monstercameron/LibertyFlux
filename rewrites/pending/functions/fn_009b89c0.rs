// original: 0x009b89c0 NativeImpl_SET_CAM_INTERP_STYLE_DETAILED
/// Engine worker behind the `SET_CAM_INTERP_STYLE_DETAILED` native.
///
/// Allocates a 28-byte interpolation-style instruction block, stamps the
/// class word and a zero flag byte, then stores five integer fields from
/// the stack arguments. Returns null when allocation fails.
export!(stdcall, rw_009b89c0(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    const CLASS: u32 = 0x00E94258;
    unsafe {
        let p = callee_cdecl!(0, u32, 0x1cu32) as *mut u8;
        if p.is_null() {
            return 0;
        }
        *(p as *mut u32) = relocated(CLASS);
        *p.add(4) = 0;
        *(p.add(8) as *mut u32) = a0;
        *(p.add(0x0c) as *mut u32) = a1;
        *(p.add(0x10) as *mut u32) = a2;
        *(p.add(0x14) as *mut u32) = a3;
        *(p.add(0x18) as *mut u32) = a4;
        p as u32
    }
});
