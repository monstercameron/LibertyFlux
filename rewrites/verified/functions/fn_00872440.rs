// original: 0x00872440 rage::crmtManager::vf0
/// Deleting destructor of the motion-tree manager.
///
/// Stamps the base-class table, releases the active child held at
/// `this + 4` when non-null (virtual slot 8 of the child's own table,
/// thiscall with no stack arguments) and clears the slot, then frees
/// `this` through the thread-local manager when the low bit of the flags
/// argument is set. Returns `this`.
///
/// Original: 0x00872440 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00872440(this: u32, flags: u32) -> u32 {
    unsafe {
        const CHILD_OFF: u32 = 4;
        const VTABLE: u32 = 0xFE7F10;
        const RELEASE_SLOT: u32 = 8;
        const FREE_FLAG: u32 = 1;
        const MANAGER_OFF: u32 = 8;
        const FREE_SLOT: u32 = 0x0C;
        let child = ((this + CHILD_OFF) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        if child != 0 {
            let child_vt = (child as *const u32).read_unaligned();
            let target = ((child_vt + RELEASE_SLOT) as *const u32).read_unaligned();
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            release(child);
            ((this + CHILD_OFF) as *mut u32).write_unaligned(0);
        }
        if flags & FREE_FLAG != 0 {
            let thread = lf_checker_rt::tls_slot(0);
            let manager = ((thread + MANAGER_OFF) as *const u32).read_unaligned();
            let vtable = (manager as *const u32).read_unaligned();
            let target = ((vtable + FREE_SLOT) as *const u32).read_unaligned();
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            free(manager, this);
        }
    }
    this
});
