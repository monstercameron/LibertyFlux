// original: 0x00697840 pool_init32 (proposed)

/// Initialise a 32-slot pool through the TLS allocator.
///
/// When the ready half at `this+0x0a` is zero it is set to `0x20` and the
/// slot array (`this+4`) is allocated (`0x80` bytes, tag `0x10`). Then the
/// stride word (`this+8`) is set to `0x20` and each of the 32 slots is
/// filled: a `0x10`-byte block is allocated and zeroed (a null block is
/// stored as-is), and a `0x400`-byte sub-block is allocated and stored at
/// block `+0x0c` (a null block faults here, exactly as the original does).
/// Finally the capacity word (`this+0x0c`) is set to `0x100`. The allocator
/// is reached through TLS slot 0 (owner at `[slot0]`, allocator at `+8`,
/// routine at vtable `+8`). No defined result.
///
/// Original: thiscall, two stack words it pops but never reads, callee
/// cleans 8.
lf_checker_rt::export!(thiscall, rw_00697840(this: u32, _u1: u32, _u2: u32) -> u32 {
    unsafe {
        const READY_HALF: u32 = 0x0a;
        const READY_VALUE: u16 = 0x20;
        const POOL_SLOTS: u32 = 4;
        const STRIDE_WORD: u32 = 8;
        const STRIDE_VALUE: u32 = 0x20;
        const CAP_WORD: u32 = 0x0c;
        const CAP_VALUE: u32 = 0x100;
        const ALLOC_SLOT: u32 = 8;
        const POOL_BYTES: u32 = 0x80;
        const BLOCK_BYTES: u32 = 0x10;
        const SUB_BYTES: u32 = 0x400;
        const BLOCK_TAG: u32 = 0x10;
        const BLOCK_SUB: u32 = 0x0c;
        const SLOT_COUNT: u32 = 32;
        let _ = (_u1, _u2);
        let owner = lf_checker_rt::tls_slot(0);
        let alloc = (owner as *const u32).byte_offset(8).read_unaligned();
        let vtable = (alloc as *const u32).read_unaligned();
        let routine =
            (vtable as *const u32).byte_offset(ALLOC_SLOT as isize).read_unaligned();
        let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(routine as usize);
        let ready = (this as *const u16).byte_offset(READY_HALF as isize).read_unaligned();
        if ready == 0 {
            (this as *mut u16).byte_offset(READY_HALF as isize).write_unaligned(READY_VALUE);
            let pool = allocate(alloc, POOL_BYTES, BLOCK_TAG, 0);
            (this as *mut u32).byte_offset(POOL_SLOTS as isize).write_unaligned(pool);
        }
        (this as *mut u16).byte_offset(STRIDE_WORD as isize).write_unaligned(STRIDE_VALUE as u16);
        let pool = (this as *const u32).byte_offset(POOL_SLOTS as isize).read_unaligned();
        let mut i: u32 = 0;
        while i < SLOT_COUNT {
            let blk = allocate(alloc, BLOCK_BYTES, BLOCK_TAG, 0);
            if blk != 0 {
                let b = blk as *mut u32;
                b.write_unaligned(0);
                b.byte_offset(4).write_unaligned(0);
                b.byte_offset(8).write_unaligned(0);
                b.byte_offset(12).write_unaligned(0);
            }
            (pool as *mut u32).byte_offset((i * 4) as isize).write_unaligned(blk);
            let sub = allocate(alloc, SUB_BYTES, BLOCK_TAG, 0);
            (blk as *mut u32).byte_offset(BLOCK_SUB as isize).write_unaligned(sub);
            i += 1;
        }
        (this as *mut u32).byte_offset(CAP_WORD as isize).write_unaligned(CAP_VALUE);
        0
    }
});
