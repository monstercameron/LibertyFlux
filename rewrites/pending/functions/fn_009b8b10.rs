// original: 0x009b8b10 NativeImpl_SET_CAM_POINT_OFFSET_IS_RELATIVE
/// Engine worker behind the `SET_CAM_POINT_OFFSET_IS_RELATIVE` native.
///
/// Allocates a 16-byte instruction block, stamps the class word and a zero
/// flag byte, stores one integer field, then stores only the low byte of
/// the second argument. Returns null when allocation fails.
export!(stdcall, rw_009b8b10(a0: u32, flag: u32) -> u32 {
    const CLASS: u32 = 0x00E94138;
    unsafe {
        let p = callee_cdecl!(0, u32, 0x10u32) as *mut u8;
        if p.is_null() {
            return 0;
        }
        *(p as *mut u32) = relocated(CLASS);
        *p.add(4) = 0;
        *(p.add(8) as *mut u32) = a0;
        *p.add(0x0c) = (flag & 0xFF) as u8;
        p as u32
    }
});
