// original: 0x00875ac0 crmt_filter_request_init_guarded
/// Initialize a filter-request object unless the pointer is null.
///
/// cdecl/1. Null-guarded form of [`rw_008759a0`]; a null pointer is a no-op.
export!(cdecl, rw_00875ac0(obj: *mut u8) -> () {
    unsafe {
        if obj.is_null() {
            return;
        }
        const TAG: u32 = 0x00130000;
        *(obj.add(0x04) as *mut u32) = TAG;
        *(obj.add(0x08) as *mut u32) = 0;
        *(obj.add(0x0c) as *mut u32) = 0;
        *(obj.add(0x10) as *mut u32) = 0;
        *(obj.add(0x14) as *mut u32) = 0;
        *(obj.add(0x18) as *mut u32) = 0;
        *(obj.add(0x1c) as *mut u32) = 0;
        *(obj as *mut u32) = relocated(0x00fe816c);
        *(obj.add(0x20) as *mut u32) = 0;
    }
});
