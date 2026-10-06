// original: 0x008725C0 rage::crmtNode::vf0
/// Deleting destructor of a motion-tree node.
///
/// Stamps the base-class table, tears the child list down through the
/// direct helper, then clears the link words at `+0x0C` and `+0x10`
/// unconditionally and the word at `+8` when it is non-null. Frees `this`
/// through the thread-local manager when the low bit of the flags
/// argument is set. Returns `this`.
///
/// Original: 0x008725C0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_008725C0(this: u32, flags: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xFE7F28;
        const TEARDOWN_CALLEE: u32 = 1;
        const FREE_FLAG: u32 = 1;
        const MANAGER_OFF: u32 = 8;
        const FREE_SLOT: u32 = 0x0C;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(TEARDOWN_CALLEE, u32, this);
        let has_link = ((this + 8) as *const u32).read_unaligned() != 0;
        ((this + 0x0C) as *mut u32).write_unaligned(0);
        ((this + 0x10) as *mut u32).write_unaligned(0);
        if has_link {
            ((this + 8) as *mut u32).write_unaligned(0);
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
