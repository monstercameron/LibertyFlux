// original: 0x00D4EC90 CTaskComplexPickUpAndCarryObject::vf1

/// Clone of CTaskComplexPickUpAndCarryObject (vf1): allocates a fresh object and constructs a copy.
///
/// Reads the game allocator pointer from its global slot and requests a
/// block (intercepted callee 1). When the allocation fails the result is
/// null. Otherwise the copy constructor runs on the new block (intercepted
/// callee 2) and copies the words at `+0x14` and `+0x18`, a pointer to the sub-object at `+0x20`, and the byte at `+0x3c`; its return value is the result.
/// The source object itself is never modified.
///
/// Original: 0x00D4EC90 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d4ec90(this: u32) -> u32 {
    unsafe {
        const ALLOCATOR_SLOT: u32 = 0x0167E2A0;
        const ALLOC: u32 = 1;
        const CTOR: u32 = 2;
        let alloc = lf_checker_rt::global::<u32>(ALLOCATOR_SLOT).read();
        let block: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, alloc);
        if block == 0 {
            return 0;
        }
        let w14 = ((this + 0x14) as *const u32).read_unaligned();
        let w18 = ((this + 0x18) as *const u32).read_unaligned();
        let lea20 = this.wrapping_add(0x20);
        let b3c = ((this + 0x3c) as *const u8).read() as u32;
        lf_checker_rt::callee_thiscall!(CTOR, u32, block, w14, w18, lea20, b3c)
    }
});
