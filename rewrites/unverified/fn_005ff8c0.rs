// original: 0x005FF8C0 rage::fragType::vf0

/// Destroy the object and, when flagged, release it to the thread allocator.
///
/// Runs the destructor, then returns `this` unless bit 0 of the stack flag
/// is set. When set (and `this` is non-null), releases the block through the
/// allocator reached from TLS slot 0 (slot value, inner object at its first
/// word, allocator at inner `+ 8`, release entry at vtable slot `+ 0xc`,
/// called with the block on the stack). The original is thiscall with one
/// stack argument.
lf_checker_rt::export!(thiscall, rw_005ff8c0(this: u32, flag: u32) -> u32 {
    unsafe {
        const DTOR_CALLEE: u32 = 1;
        const ALLOC_LINK: u32 = 8;
        const FREE_SLOT: u32 = 0x0c;
        lf_checker_rt::callee_thiscall!(DTOR_CALLEE, u32, this);
        if flag & 1 == 0 {
            return this;
        }
        if this == 0 {
            return 0;
        }
        let tls = lf_checker_rt::tls_slot(0);
        let inner = (tls as *const u32).read_unaligned();
        let alloc = (inner.wrapping_add(ALLOC_LINK) as *const u32)
            .read_unaligned();
        let vtable = (alloc as *const u32).read_unaligned();
        let entry = (vtable.wrapping_add(FREE_SLOT) as *const u32)
            .read_unaligned();
        let free_fn: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(entry as usize);
        free_fn(alloc, this);
        this
    }
});
