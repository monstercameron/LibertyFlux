// original: 0x00875b10 crmt_filter_request_set_vtable_guarded
/// Install the filter-request vtable unless the pointer is null.
///
/// cdecl/1. A null pointer is a no-op.
export!(cdecl, rw_00875b10(obj: *mut u8) -> () {
    unsafe {
        if obj.is_null() {
            return;
        }
        *(obj as *mut u32) = relocated(0x00fe816c);
    }
});
