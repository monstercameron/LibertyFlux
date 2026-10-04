// original: 0x00a64dd0 tracked_object_release
/// Releases the tracked object at +0x25C when one is present.
///
/// Calls the object's release slot (vtable +0) with argument 1, then clears
/// the slot. Returns nothing.
export!(thiscall, rw_00a64dd0(this: u32) -> u32 {
    unsafe {
        let obj = *((this + 0x25C) as *const u32);
        if obj == 0 {
            return 0;
        }
        type Release = extern "thiscall" fn(u32, u32) -> u32;
        let vtable = *(obj as *const u32);
        let release: Release =
            core::mem::transmute(*(vtable as *const u32) as usize);
        release(obj, 1);
        *((this + 0x25C) as *mut u32) = 0;
    }
    0
});
