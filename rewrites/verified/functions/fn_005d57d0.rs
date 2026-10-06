// original: 0x005d57d0 CMissionCleanup::vf0

/// Deleting destructor for the mission-cleanup record.
///
/// Stamps the vtable, then frees the object through the thread allocator
/// (reached from TLS slot 0: allocator at `+8`, vtable at `+0`, free at
/// vtable `+0x0c`, called with the allocator in ECX and the pointer on the
/// stack) when the low bit of the `flags` word is set. Returns the object.
///
/// Original: 0x005d57d0 (thiscall, one stack word: deleting flags).
lf_checker_rt::export!(thiscall, rw_005d57d0(this: u32, flags: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E99FE0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(this, lf_checker_rt::relocated(VTABLE));
        if flags & 1 != 0 {
            let thread = lf_checker_rt::tls_slot(0);
            let allocator = rd32(thread + 8);
            let vtable = rd32(allocator);
            let free_fn: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vtable + 0x0c) as usize);
            free_fn(allocator, this);
        }
        this
    }
});
