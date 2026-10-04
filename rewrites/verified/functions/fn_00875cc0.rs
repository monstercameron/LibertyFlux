// original: 0x00875cc0 crmt_extrapolate_node_init_guarded
/// Initialize an extrapolate-node object unless the pointer is null.
///
/// cdecl/1. Null-guarded form of [`rw_00875bc0`]; the original reaches the
/// shared initializer with a conditional tail jump, which is equivalent to
/// this branch. A null pointer is a no-op.
export!(cdecl, rw_00875cc0(obj: *mut u8) -> () {
    unsafe {
        if obj.is_null() {
            return;
        }
        const TAG: u32 = 0x00120000;
        const ONE: u32 = 0x3f800000;
        *(obj.add(0x04) as *mut u32) = TAG;
        *(obj.add(0x08) as *mut u32) = 0;
        *(obj.add(0x0c) as *mut u32) = 0;
        *(obj.add(0x10) as *mut u32) = 0;
        *(obj.add(0x14) as *mut u32) = 0;
        *(obj.add(0x18) as *mut u32) = 0;
        *(obj.add(0x1c) as *mut u32) = 0;
        *(obj as *mut u32) = relocated(0x00fe81a8);
        *(obj.add(0x20) as *mut u32) = ONE;
        *(obj.add(0x24) as *mut u32) = 0;
        *(obj.add(0x28) as *mut u32) = 0;
        *(obj.add(0x2c) as *mut u16) = 0x0101;
        *(obj.add(0x30) as *mut u32) = 0;
    }
});
