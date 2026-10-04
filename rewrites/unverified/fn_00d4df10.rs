// original: 0x00D4DF10 CTaskSimpleTakeOffHelmet::vf1

/// Clone of CTaskSimpleTakeOffHelmet (vf1): allocates a fresh object and constructs a copy.
///
/// Reads the game allocator pointer from its global slot and requests a
/// block (intercepted callee 1). When the allocation fails the result is
/// null. Otherwise the copy constructor runs on the new block (intercepted
/// callee 2, reached by a tail jump in the original) and takes no further arguments (tail call); its return value is the result.
/// The source object itself is never modified.
///
/// Original: 0x00D4DF10 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d4df10(this: u32) -> u32 {
    unsafe {
        const ALLOCATOR_SLOT: u32 = 0x0167E2A0;
        const ALLOC: u32 = 1;
        const CTOR: u32 = 2;
        let alloc = lf_checker_rt::global::<u32>(ALLOCATOR_SLOT).read();
        let block: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, alloc);
        if block == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CTOR, u32, block)
    }
});
