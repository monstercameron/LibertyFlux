// original: 0x00d2b2c0 task_ensure_block (proposed)
/// Ensure the block pointer at `this+0x74`: when null, allocate a 0x20-byte
/// block, zero its words at `+0x10`/`+0x14`, store and return it (null when
/// allocation fails). When already set, the original returns whatever was in
/// eax, which no caller can depend on; the contract only covers the null
/// path (see `narrowed`).
///
/// Thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00d2b2c0(this: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x74;
        const ALLOC_BYTES: u32 = 0x20;
        const ALLOC: u32 = 1;
        let slot = this + SLOT;
        if (slot as *const u32).read_unaligned() != 0 {
            return (slot as *const u32).read_unaligned();
        }
        let blk: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, ALLOC_BYTES);
        if blk == 0 {
            (slot as *mut u32).write_unaligned(0);
            0
        } else {
            ((blk + 0x10) as *mut u32).write_unaligned(0);
            ((blk + 0x14) as *mut u32).write_unaligned(0);
            (slot as *mut u32).write_unaligned(blk);
            blk
        }
    }
});
