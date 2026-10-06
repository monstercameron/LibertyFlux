// original: 0x005d5960 CHtmlNode::vf0

/// Deleting destructor for the base HTML node.
///
/// Runs the child-list teardown callee on the object, then frees the object
/// through the thread allocator (reached from TLS slot 0: allocator at `+8`,
/// vtable at `+0`, free at vtable `+0x0c`, called with the allocator in ECX
/// and the pointer on the stack) when the low bit of the `flags` word is set
/// and the object pointer is non-null. Returns the object.
///
/// Original: 0x005d5960 (thiscall, one stack word: deleting flags).
lf_checker_rt::export!(thiscall, rw_005d5960(this: u32, flags: u32) -> u32 {
    unsafe {
        const TEARDOWN_CALLEE: u32 = 1;
        const FREE_OFF: u32 = 0x0c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        lf_checker_rt::callee_thiscall!(TEARDOWN_CALLEE, u32, this);
        if flags & 1 != 0 && this != 0 {
            let thread = lf_checker_rt::tls_slot(0);
            let allocator = rd32(thread + 8);
            let vtable = rd32(allocator);
            let free_fn: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vtable + FREE_OFF) as usize);
            free_fn(allocator, this);
        }
        this
    }
});
