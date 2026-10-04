// original: 0x0069a4b0 install_vtable_fe3c94
/// Install the channel vtable at `obj`, doing nothing for a null pointer.
/// Returns the pointer it was given.
export!(cdecl, rw_0069a4b0(obj: u32) -> u32 {
    unsafe {
        if obj != 0 {
            *(obj as *mut u32) = relocated(0xFE3C94);
        }
        obj
    }
});
