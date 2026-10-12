// original: 0x005FC2B0 rage::atAny::Holder<int>::vf2

/// Clone this holder through the thread allocator.
///
/// Allocates a `8`-byte block through the allocator reached from TLS slot 0
/// (slot value, inner object at its first word, allocator at inner `+ 8`,
/// allocate entry at vtable slot `+ 8`, called with size, alignment 16 and
/// flags 0). A failed allocation returns null. Otherwise installs the holder
/// vtable and copies the 1 payload word from `this + 4`, returning the
/// new block. The original is thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_005fc2b0(this: u32) -> u32 {
    unsafe {
        const ALLOC_LINK: u32 = 8;
        const ALLOC_SLOT: u32 = 8;
        const BLOCK_SIZE: u32 = 0x8;
        const BLOCK_ALIGN: u32 = 0x10;
        const VTABLE: u32 = 0x00FE17C4;
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
        (block as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE));
        let w0 = (this.wrapping_add(0x04) as *const u32).read_unaligned();
        (block.wrapping_add(0x04) as *mut u32).write_unaligned(w0);
        block
    }
});
