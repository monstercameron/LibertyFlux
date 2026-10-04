// original: 0x00a64e20 tracked_object_poll_release
/// Polls the tracked object, releasing it when the poll reports false.
///
/// Returns 0 when the slot at +0x25C is empty. Otherwise polls the object;
/// a true poll keeps the slot, a false poll releases the object through its
/// release slot and clears it. Returns 1 in both non-empty cases.
export!(thiscall, rw_00a64e20(this: u32) -> u32 {
    unsafe {
        let obj = *((this + 0x25C) as *const u32);
        if obj == 0 {
            return 0;
        }
        let poll = callee_thiscall!(1, u32, obj);
        if (poll as u8) != 0 {
            return 1;
        }
        let obj = *((this + 0x25C) as *const u32);
        if obj != 0 {
            type Release = extern "thiscall" fn(u32, u32) -> u32;
            let vtable = *(obj as *const u32);
            let release: Release =
                core::mem::transmute(*(vtable as *const u32) as usize);
            release(obj, 1);
            *((this + 0x25C) as *mut u32) = 0;
        }
    }
    1
});
