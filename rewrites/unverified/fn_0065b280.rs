// original: 0x0065B280 tls_alloc_d0_block (proposed)

/// Allocate a 0xd0-byte block through the thread-local allocator and init it.
///
/// Same allocator call as the `vf5` family (TLS slot 0, slot `+8`, size
/// `0xd0`, align `0x10`, flags 0). On success writes the vtable pointer at
/// the block start and zeroes the dwords at `+0xc4`, `+0xc8`, `+0xcc` and the
/// bytes at `+0x44` and `+0x04`. Returns the block, or null when allocation
/// fails (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_0065b280() -> u32 {
    unsafe {
        const ALLOC_SIZE: u32 = 0xD0;
        const VTABLE: u32 = 0xFE2E3C;
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
            ((mem + 0xc4) as *mut u32).write_unaligned(0);
            ((mem + 0xc8) as *mut u32).write_unaligned(0);
            ((mem + 0xcc) as *mut u32).write_unaligned(0);
            ((mem + 0x44) as *mut u8).write_unaligned(0);
            ((mem + 0x04) as *mut u8).write_unaligned(0);
            mem
        }
    }
});
