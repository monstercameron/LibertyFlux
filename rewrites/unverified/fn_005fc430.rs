// original: 0x005FC430 holder_attach_copy (proposed)

/// Attach a fresh iterator-holder copy built from the source argument.
///
/// Allocates a 20-byte block through the allocator reached from TLS slot 0
/// (slot value, inner object at its first word, allocator at inner `+ 8`,
/// allocate entry at vtable slot `+ 8`, called with size, alignment 16 and
/// flags 0), copying the four source words into the block past its vtable.
/// Stores the new block (or null when the allocation fails) at `this` and
/// returns `this`. The original is thiscall with one stack argument.
lf_checker_rt::export!(thiscall, rw_005fc430(this: u32, src: u32) -> u32 {
    unsafe {
        const ALLOC_LINK: u32 = 8;
        const ALLOC_SLOT: u32 = 8;
        const BLOCK_SIZE: u32 = 0x14;
        const BLOCK_ALIGN: u32 = 0x10;
        const VTABLE: u32 = 0x00FE1764;
        const PAYLOAD: u32 = 4;
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
            (this as *mut u32).write_unaligned(0);
            return this;
        }
        (block as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE));
        let mut i = 0u32;
        while i < PAYLOAD {
            let w = (src.wrapping_add(i.wrapping_mul(4)) as *const u32)
                .read_unaligned();
            (block.wrapping_add(4u32.wrapping_add(i.wrapping_mul(4)))
                as *mut u32)
                .write_unaligned(w);
            i += 1;
        }
        (this as *mut u32).write_unaligned(block);
        this
    }
});
