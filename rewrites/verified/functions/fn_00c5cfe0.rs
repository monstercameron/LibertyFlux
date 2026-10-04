// original: 0x00c5cfe0 task_copy_sixteen_bytes (proposed)

/// Copy the four argument words into a freshly allocated sixteen-byte block.
///
/// Invokes callee 1 with the global allocator in ecx and the size 0x10, then
/// copies the four incoming words to the returned address and returns it.
///
/// Original: 0x00c5cfe0 (cdecl, four stack words).
lf_checker_rt::export!(cdecl, rw_00c5cfe0(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const ALLOCATOR: u32 = 0x16dce68;
        const SIZE: u32 = 0x10;
        const ALLOC: u32 = 1;
        let dst: u32 = lf_checker_rt::callee_thiscall!(
            ALLOC,
            u32,
            lf_checker_rt::relocated(ALLOCATOR),
            SIZE
        );
        ((dst) as *mut u32).write_unaligned(a0);
        ((dst + 4) as *mut u32).write_unaligned(a1);
        ((dst + 8) as *mut u32).write_unaligned(a2);
        ((dst + 12) as *mut u32).write_unaligned(a3);
        dst
    }
});

