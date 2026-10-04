// original: 0x00d27f90 CPedTargetting::vf0 (symbols)

/// Deleting destructor: tear down and free when asked.
///
/// Runs the destructor body (intercepted) on `this`; when the low bit of
/// `flags` is set, also frees `this` through the global allocator with the
/// heap-free helper (intercepted). Returns `this`.
///
/// Original: 0x00D27F90 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00d27f90(this: u32, flags: u32) -> u32 {
    unsafe {
        const ALLOCATOR_SLOT: u32 = 0x0171_f9c0;
        const FREE_BIT: u32 = 1;
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
        if flags & FREE_BIT != 0 {
            let alloc = unsafe { (lf_checker_rt::relocated(ALLOCATOR_SLOT) as *const u32).read_unaligned() };
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, alloc, this);
        }
        this
    }
});
