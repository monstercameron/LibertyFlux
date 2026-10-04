// original: 0x00D4F330 CTaskComplexPickUpAndCarryObject::vf19

// Refresh handler (vf19): reuses the carried object or clones a replacement.
///
/// Reads the carried pointer at `+0x14`; a null pointer yields null. Otherwise
/// the tag nibble at target `+0x1e2` is tested: below 2, or with the word at
/// target `+0x1bc` differing from the argument, a replacement is cloned (game
/// allocator through intercepted callee 2, constructor through intercepted
/// callee 3 with the words at `+0x18` and `+0x14`), and a failed allocation
/// yields null. When the tag is 2 or more and the word matches, intercepted
/// callee 1 runs on `this` instead and its result is returned. (The original
/// re-reads the tag before loading the word; nothing can change it between
/// the reads, so one read is enough.)
///
/// Original: 0x00D4F330 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00d4f330(this: u32, arg0: u32) -> u32 {
    unsafe {
        const ALLOCATOR_SLOT: u32 = 0x0167E2A0;
        const REFRESH: u32 = 1;
        const ALLOC: u32 = 2;
        const CTOR: u32 = 3;
        const TAG_OFF: u32 = 0x1e2;
        const WORD_OFF: u32 = 0x1bc;
        let target = ((this + 0x14) as *const u32).read_unaligned();
        if target == 0 {
            return 0;
        }
        let tag = ((target + TAG_OFF) as *const u8).read() & 0x0f;
        let cur = ((target + WORD_OFF) as *const u32).read_unaligned();
        if tag < 2 || cur != arg0 {
            let alloc = lf_checker_rt::global::<u32>(ALLOCATOR_SLOT).read();
            let block: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, alloc);
            if block == 0 {
                return 0;
            }
            let w18 = ((this + 0x18) as *const u32).read_unaligned();
            let w14 = ((this + 0x14) as *const u32).read_unaligned();
            return lf_checker_rt::callee_thiscall!(CTOR, u32, block, w14, w18);
        }
        lf_checker_rt::callee_thiscall!(REFRESH, u32, this)
    }
});
