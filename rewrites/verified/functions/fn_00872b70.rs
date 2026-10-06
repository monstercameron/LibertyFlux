// original: 0x00872B70 rage::crmtNodeParent::vf0
/// Deleting destructor of a parent motion-tree node.
///
/// Runs the direct dtor body on `this`, then frees `this` through the
/// thread-local manager when the low bit of the flags argument is set and
/// `this` is non-null. Returns `this`.
///
/// Original: 0x00872B70 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00872B70(this: u32, flags: u32) -> u32 {
    unsafe {
        const DTOR_CALLEE: u32 = 1;
        const FREE_FLAG: u32 = 1;
        const MANAGER_OFF: u32 = 8;
        const FREE_SLOT: u32 = 0x0C;
        lf_checker_rt::callee_thiscall!(DTOR_CALLEE, u32, this);
        if flags & FREE_FLAG != 0 && this != 0 {
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
