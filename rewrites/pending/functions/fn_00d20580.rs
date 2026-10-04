// original: 0x00d20580 CTaskComplexSeekCoverShooting::vf5
// Initialises the shooting task against its subject: unless the subject
// is already prepared, asks it through its vtable and fails (returning 0)
// when it declines, marking it prepared when it accepts. Then resets the
// two member trackers and the first argument's object. Returns 1 on
// success, 0 when the subject declines.
export!(thiscall, rw_00d20580(this_ptr: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        let sub = *((this_ptr.wrapping_add(8)) as *const u32);
        if (*((sub.wrapping_add(0xC)) as *const u8) & 1) == 0 {
            let vtable = *(sub as *const u32);
            let target = *((vtable.wrapping_add(0x14)) as *const u32);
            let ask: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            if (ask(sub, a0, a1, a2) & 0xFF) == 0 {
                return 0;
            }
            let flags = *((sub.wrapping_add(0xC)) as *const u32);
            *((sub.wrapping_add(0xC)) as *mut u32) = flags | 2;
        }
        callee_thiscall!(2, u32, this_ptr.wrapping_add(0x60));
        callee_thiscall!(3, u32, this_ptr.wrapping_add(0x6C));
        callee_thiscall!(4, u32, a0);
        1
    }
});
