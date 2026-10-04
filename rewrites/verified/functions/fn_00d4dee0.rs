// original: 0x00D4DEE0 CTaskSimpleShakeFist::vf1

/// Clone of CTaskSimpleShakeFist (vf1): allocates a fresh object and constructs a copy.
///
/// Reads the game allocator pointer from its global slot and requests a
/// block (intercepted callee 1). When the allocation fails the result is
/// null. Otherwise the copy constructor runs on the new block (intercepted
/// callee 2) and copies the word at `+0x1c`; its return value is the result.
/// The source object itself is never modified.
///
/// Original: 0x00D4DEE0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d4dee0(this: u32) -> u32 {
    unsafe {
        const ALLOCATOR_SLOT: u32 = 0x0167E2A0;
        const ALLOC: u32 = 1;
        const CTOR: u32 = 2;
        let alloc = lf_checker_rt::global::<u32>(ALLOCATOR_SLOT).read();
        let block: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, alloc);
        if block == 0 {
            return 0;
        }
        let w1c = ((this + 0x1c) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(CTOR, u32, block, w1c)
    }
});
