// original: 0x0065B1D0 tls_alloc_pass_block (proposed)

/// Allocate a 0x8c-byte pass block through the TLS allocator and init it.
///
/// Calls the allocator (TLS slot 0, slot `+8`, thiscall: object, `0x8c`,
/// `0x10`, 0) and returns null on failure. On success stamps the vtable
/// pointer at `+0`, the dwords `-1`, `4`, `0x280`, `0x1e0`, `0x20` at
/// `+0x44`..`+0x54`, a zero byte at `+0x58`, then runs the init callee
/// (thiscall on the block `+0x5c`: flags 0) and zeroes the byte at `+4`.
/// Returns the block (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_0065b1d0() -> u32 {
    unsafe {
        const ALLOC_SIZE: u32 = 0x8C;
        const VTABLE: u32 = 0xFE2E48;
        const CALLEE_INIT: u32 = 2;
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
            ((mem + 0x44) as *mut u32).write_unaligned(0xFFFFFFFF);
            ((mem + 0x48) as *mut u32).write_unaligned(4);
            ((mem + 0x4c) as *mut u32).write_unaligned(0x280);
            ((mem + 0x50) as *mut u32).write_unaligned(0x1e0);
            ((mem + 0x54) as *mut u32).write_unaligned(0x20);
            ((mem + 0x58) as *mut u8).write_unaligned(0);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CALLEE_INIT, u32, mem.wrapping_add(0x5c), 0);
            ((mem + 4) as *mut u8).write_unaligned(0);
            mem
        }
    }
});
