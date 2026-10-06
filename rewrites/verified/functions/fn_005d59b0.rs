// original: 0x005d59b0 CHtmlTableElementNode::vf0

/// Deleting destructor for the HTML table-element node.
///
/// Frees the owned style block at `+0xe0` through the thread allocator
/// (reached from TLS slot 0: allocator at `+8`, vtable at `+0`, free at
/// vtable `+0x0c`) when it is non-null, runs the base-node teardown callee,
/// then frees the object itself through the same allocator when the low bit
/// of the `flags` word is set. Returns the object.
///
/// Original: 0x005d59b0 (thiscall, one stack word: deleting flags).
lf_checker_rt::export!(thiscall, rw_005d59b0(this: u32, flags: u32) -> u32 {
    unsafe {
        const STYLE_OFF: u32 = 0xe0;
        const TEARDOWN_CALLEE: u32 = 1;
        const FREE_OFF: u32 = 0x0c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn release(ptr: u32) {
            unsafe {
                let thread = lf_checker_rt::tls_slot(0);
                let allocator = rd32(thread + 8);
                let vtable = rd32(allocator);
                let free_fn: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vtable + FREE_OFF) as usize);
                free_fn(allocator, ptr);
            }
        }

        let style = rd32(this + STYLE_OFF);
        if style != 0 {
            release(style);
        }
        lf_checker_rt::callee_thiscall!(TEARDOWN_CALLEE, u32, this);
        if flags & 1 != 0 {
            release(this);
        }
        this
    }
});
