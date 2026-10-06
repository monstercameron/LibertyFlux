// original: 0x005d6240 CHtmlTableNode::vf0

/// Deleting destructor for the HTML table node.
///
/// Stamps the vtable, runs the grid teardown callee, frees the owned caption
/// at `+0xe0` through the thread allocator (reached from TLS slot 0:
/// allocator at `+8`, vtable at `+0`, free at vtable `+0x0c`) when it is
/// non-null, runs the base-node teardown callee, then frees the object
/// itself through the same allocator when the low bit of the `flags` word is
/// set. Returns the object.
///
/// Original: 0x005d6240 (thiscall, one stack word: deleting flags).
lf_checker_rt::export!(thiscall, rw_005d6240(this: u32, flags: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00FE0B50;
        const CAPTION_OFF: u32 = 0xe0;
        const GRID_CALLEE: u32 = 1;
        const TEARDOWN_CALLEE: u32 = 3;
        const FREE_OFF: u32 = 0x0c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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

        wr32(this, lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(GRID_CALLEE, u32, this);
        let caption = rd32(this + CAPTION_OFF);
        if caption != 0 {
            release(caption);
        }
        lf_checker_rt::callee_thiscall!(TEARDOWN_CALLEE, u32, this);
        if flags & 1 != 0 {
            release(this);
        }
        this
    }
});
