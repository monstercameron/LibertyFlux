// original: 0x00697fb0 mover_alloc_construct (proposed)

/// Allocate a mover through the TLS allocator, then construct it in place.
///
/// Asks the allocator (TLS slot 0, owner at `[slot0]`, allocator at `+8`,
/// allocate routine at vtable `+8`) for a `0x60`-byte block with tag `0x10`.
/// A null answer returns null. Otherwise control passes to the constructor
/// with the block as its object; the original reaches it by tail jump, the
/// rewrite by call, and its result is returned.
///
/// Original: no stack words, entry `ecx` ignored, plain return (cdecl shape);
/// the tail-jump target likewise pops nothing.
lf_checker_rt::export!(cdecl, rw_00697fb0() -> u32 {
    unsafe {
        const ALLOC_SLOT: u32 = 8;
        const BLOCK_SIZE: u32 = 0x60;
        const BLOCK_TAG: u32 = 0x10;
        const CTOR: u32 = 2;
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
        lf_checker_rt::callee_thiscall!(CTOR, u32, block)
    }
});
