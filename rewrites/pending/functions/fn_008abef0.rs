// original: 0x008ABEF0 rage::audBiquadFilterEffect::vf0
/// Deleting destructor: release the extra buffer, restore the class
/// vtable, run the base destructor, and free the object when deleting.
export!(thiscall, rw_008ABEF0(obj: *mut u8, deleting: u32) -> u32 {
    unsafe {
        let extra = *(obj.add(0x74) as *const u32);
        *(obj as *mut u32) = relocated(0x00E7C548);
        callee_cdecl!(1, u32, extra);
        callee_thiscall!(2, u32, obj as u32);
        if deleting & 1 != 0 {
            callee_cdecl!(1, u32, obj as u32);
        }
        obj as u32
    }
});
