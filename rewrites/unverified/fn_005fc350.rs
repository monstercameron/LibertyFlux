// original: 0x005FC350 rage::atAny::Holder<rage::Matrix34>::vf2

/// Clone this matrix holder through the thread allocator and its builder.
///
/// Allocates an 80-byte block through the allocator reached from TLS slot 0
/// (slot value, inner object at its first word, allocator at inner `+ 8`,
/// allocate entry at vtable slot `+ 8`, called with size, alignment 16 and
/// flags 0). A failed allocation returns null. Otherwise initialises the
/// block by calling the matrix builder with the new block as `this` and
/// `this + 0x10` on the stack, returning the builder's result. The original
/// is thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_005fc350(this: u32) -> u32 {
    unsafe {
        const ALLOC_LINK: u32 = 8;
        const ALLOC_SLOT: u32 = 8;
        const BLOCK_SIZE: u32 = 0x50;
        const BLOCK_ALIGN: u32 = 0x10;
        const SOURCE_OFFSET: u32 = 0x10;
        const BUILD_CALLEE: u32 = 2;
        let tls = lf_checker_rt::tls_slot(0);
        let inner = (tls as *const u32).read_unaligned();
        let alloc = (inner.wrapping_add(ALLOC_LINK) as *const u32)
            .read_unaligned();
        let vtable = (alloc as *const u32).read_unaligned();
        let entry = (vtable.wrapping_add(ALLOC_SLOT) as *const u32)
            .read_unaligned();
        let alloc_fn: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(entry as usize);
        let block = alloc_fn(alloc, BLOCK_SIZE, BLOCK_ALIGN, 0);
        if block == 0 {
            return 0;
        }
        let source = this.wrapping_add(SOURCE_OFFSET);
        lf_checker_rt::callee_thiscall!(BUILD_CALLEE, u32, block, source)
    }
});
