// original: 0x0065B2D0 tls_alloc_28_block (proposed)

/// Allocate a 0x28-byte block through the thread-local allocator and clear it.
///
/// Same allocator call as the `vf5` family (TLS slot 0, slot `+8`, size
/// `0x28`, align `0x10`, flags 0). On success writes the vtable pointer at
/// the block start and zeroes every word from `+0x04` through `+0x20` plus
/// the byte at `+0x24`. Returns the block, or null when allocation fails
/// (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_0065b2d0() -> u32 {
    unsafe {
        const ALLOC_SIZE: u32 = 0x28;
        const VTABLE: u32 = 0xFE2424;
        let holder = lf_checker_rt::tls_slot(0);
        let obj = ((holder + 8) as *const u32).read_unaligned();
        let vtable = (obj as *const u32).read_unaligned();
        let tgt = ((vtable + 8) as *const u32).read_unaligned();
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let mem = alloc(obj, ALLOC_SIZE, 0x10, 0);
        if mem == 0 {
            0
        } else {
            (mem as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
            let mut i = 1u32;
            while i < 9 {
                ((mem + i * 4) as *mut u32).write_unaligned(0);
                i += 1;
            }
            ((mem + 0x24) as *mut u8).write_unaligned(0);
            mem
        }
    }
});
