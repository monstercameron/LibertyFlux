// original: 0x0069a5e0 install_vtable_fe3d9c
/// Install the channel vtable at `obj`, doing nothing for a null pointer.
/// Returns the pointer it was given.
export!(cdecl, rw_0069a5e0(obj: u32) -> u32 {
    unsafe {
        if obj != 0 {
            *(obj as *mut u32) = relocated(0xFE3D9C);
        }
        obj
    }
});
