// original: 0x00697ff0 rage::crCreatureComponentMover::vf11

/// Clone a mover component through the TLS allocator.
///
/// Asks the allocator (TLS slot 0, owner at `[slot0]`, allocator at `+8`,
/// allocate routine at vtable `+8`) for a `0x60`-byte block with tag `0x10`.
/// A null answer means the clone fails and the function returns null.
/// Otherwise the new block is filled by the copy helper with `this` as the
/// source, and the helper's result (the new block) is returned.
///
/// Original: thiscall, no stack words, plain return.
lf_checker_rt::export!(thiscall, rw_00697ff0(this: u32) -> u32 {
    unsafe {
        const ALLOC_SLOT: u32 = 8;
        const BLOCK_SIZE: u32 = 0x60;
        const BLOCK_TAG: u32 = 0x10;
        const COPY_HELPER: u32 = 2;
        let owner = lf_checker_rt::tls_slot(0);
        let alloc = (owner as *const u32).byte_offset(8).read_unaligned();
        let vtable = (alloc as *const u32).read_unaligned();
        let routine = (vtable as *const u32).byte_offset(ALLOC_SLOT as isize).read_unaligned();
        let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(routine as usize);
        let block = allocate(alloc, BLOCK_SIZE, BLOCK_TAG, 0);
        if block == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(COPY_HELPER, u32, block, this)
    }
});
