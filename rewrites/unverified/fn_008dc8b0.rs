// original: 0x008DC8B0 CDrawCommandAllocator::vf2

/// CDrawCommandAllocator::vf2 (draw-command virtual slot 2).
///
/// Allocates a zero-filled command block of `size` bytes from the
/// global allocator object (second slot argument 0) and returns the
/// block pointer. The two trailing arguments are ignored.
///
/// Original: 0x008DC8B0 (thiscall: `this` in ECX, `size`, two ignored).
lf_checker_rt::export!(thiscall, rw_008dc8b0(this: u32, size: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const ALLOCATOR: u32 = 0x01175C58;
        let _ = this;
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(ALLOCATOR), 0, size)
    }
});
