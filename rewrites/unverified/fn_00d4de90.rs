// original: 0x00D4DE90 CTaskSimpleDuck::vf1

/// Clone of CTaskSimpleDuck (vf1): allocates a fresh object and constructs a copy.
///
/// Reads the game allocator pointer from its global slot and requests a
/// block (intercepted callee 1). When the allocation fails the result is
/// null. Otherwise the copy constructor runs on the new block (intercepted
/// callee 2) and copies the signed word at `+0x1c`, the word at `+0x18` and the byte at `+0x38`; its return value is the result.
/// The source object itself is never modified.
///
/// Original: 0x00D4DE90 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d4de90(this: u32) -> u32 {
    unsafe {
        const ALLOCATOR_SLOT: u32 = 0x0167E2A0;
        const ALLOC: u32 = 1;
        const CTOR: u32 = 2;
        let alloc = lf_checker_rt::global::<u32>(ALLOCATOR_SLOT).read();
        let block: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, alloc);
        if block == 0 {
            return 0;
        }
        let b38 = ((this + 0x38) as *const u8).read() as u32;
        let w18 = ((this + 0x18) as *const u32).read_unaligned();
        let sx1c = ((this + 0x1c) as *const i16).read_unaligned() as i32 as u32;
        lf_checker_rt::callee_thiscall!(CTOR, u32, block, b38, w18, sx1c)
    }
});
