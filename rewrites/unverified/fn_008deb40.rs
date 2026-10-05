// original: 0x008deb40 aligned_alloc_forward (proposed)

/// Round `base` up to 16 bytes and allocate `size` through the arena.
///
/// Aligns `base` (`base + ((-base) mod 16)`) and forwards it with `size` to
/// the arena allocator (callee 1, called with the arena singleton), whose
/// answer is returned. Cdecl, two stack arguments, one outgoing call.
lf_checker_rt::export!(cdecl, rw_008deb40(base: u32, size: u32) -> u32 {
    unsafe {
        const ARENA: u32 = 0x0117_5c58;
        const CALLEE_ALLOC: u32 = 1;
        const ALIGN_MASK: u32 = 0x0f;
        let aligned = base.wrapping_add(base.wrapping_neg() & ALIGN_MASK);
        lf_checker_rt::callee_thiscall!(
            CALLEE_ALLOC,
            u32,
            lf_checker_rt::relocated(ARENA),
            aligned,
            size
        )
    }
});
