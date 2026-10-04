// original: 0x008AC200 rage::audReverbEffect::vf0
/// Deleting destructor: release the extra buffer, restore the class
/// vtable, run the base destructor, and free the object when deleting.
export!(thiscall, rw_008AC200(obj: *mut u8, deleting: u32) -> u32 {
    unsafe {
        let extra = *(obj.add(0xC8) as *const u32);
        *(obj as *mut u32) = relocated(0x00E7C59C);
        callee_cdecl!(1, u32, extra);
        callee_thiscall!(2, u32, obj as u32);
        if deleting & 1 != 0 {
            callee_cdecl!(1, u32, obj as u32);
        }
        obj as u32
    }
});
