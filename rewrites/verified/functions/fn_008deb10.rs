// original: 0x008deb10 alloc_and_process (proposed)

/// Allocate an aligned block for `size` and process it with `key`.
///
/// Aligns `size` up to 16 bytes, allocates through the arena (callee 1,
/// called with the arena singleton and `flags`), then calls the processor
/// (callee 2) with `key`, the allocated pointer and `size`, returning its
/// answer. Cdecl, three stack arguments, two outgoing calls.
lf_checker_rt::export!(cdecl, rw_008deb10(key: u32, size: u32, flags: u32) -> u32 {
    unsafe {
        const ARENA: u32 = 0x0117_5c58;
        const CALLEE_ALLOC: u32 = 1;
        const CALLEE_PROCESS: u32 = 2;
        const ALIGN_MASK: u32 = 0x0f;
        let aligned = size.wrapping_add(size.wrapping_neg() & ALIGN_MASK);
        let block = lf_checker_rt::callee_thiscall!(
            CALLEE_ALLOC,
            u32,
            lf_checker_rt::relocated(ARENA),
            aligned,
            flags
        );
        lf_checker_rt::callee_cdecl!(CALLEE_PROCESS, u32, key, block, size)
    }
});
