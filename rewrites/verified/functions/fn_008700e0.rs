// original: 0x008700E0 rage::crmtComposerOptimizedData::vf0

/// Scalar-deleting destructor: run the member teardown (callee 1) on `this`,
/// then free `this` through the thread-local manager when the low flag bit
/// is set and `this` is non-null (TLS slot 0 points at the thread block,
/// whose word at +8 is the manager; the free routine is vtable slot +0xC,
/// taking `this` as its stack argument). Both guards are exact bit/null
/// tests, not signed comparisons. Returns `this`.
///
/// Original: 0x008700E0 (thiscall, one stack word; the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_008700e0(this: u32, flags: u32) -> u32 {
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const TEARDOWN: u32 = 1;
    lf_checker_rt::callee_thiscall!(TEARDOWN, u32, this);
    if flags & 1 != 0 && this != 0 {
        unsafe {
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
