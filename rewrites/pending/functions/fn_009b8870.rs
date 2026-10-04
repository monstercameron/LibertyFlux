// original: 0x009b8870 NativeImpl_SET_CAM_INTERP_DETAIL_ROT_STYLE_ANGLES
/// Engine worker behind the `SET_CAM_INTERP_DETAIL_ROT_STYLE_ANGLES` native.
///
/// Allocates a 12-byte rotation-style instruction block, stamps the class
/// word and a zero flag byte, then stores one integer field from the stack
/// argument. Returns null when allocation fails.
export!(stdcall, rw_009b8870(a0: u32) -> u32 {
    const CLASS: u32 = 0x00E94268;
    unsafe {
        let p = callee_cdecl!(0, u32, 0x0cu32) as *mut u8;
        if p.is_null() {
            return 0;
        }
        *(p as *mut u32) = relocated(CLASS);
        *p.add(4) = 0;
        *(p.add(8) as *mut u32) = a0;
        p as u32
    }
});
