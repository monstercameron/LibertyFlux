// original: 0x00875cd0 crmt_extrapolate_node_set_vtable_guarded
/// Install the extrapolate-node vtable unless the pointer is null.
///
/// cdecl/1. A null pointer is a no-op.
export!(cdecl, rw_00875cd0(obj: *mut u8) -> () {
    unsafe {
        if obj.is_null() {
            return;
        }
        *(obj as *mut u32) = relocated(0x00fe81a8);
    }
});
