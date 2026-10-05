// original: 0x00887270 stream_global_init (proposed)

/// Allocate the stream tables and publish them through globals.
///
/// Reads three dimension words from globals, allocates (callee 1, twice)
/// the slot array (`dim0 * 16` bytes) and the index block
/// (`dim1 * 4 + 0x20` bytes), stores the slot array at `[this]`, the span
/// `dim1 - dim2` at `[this+0xc]` and zeroes at `[this+0x10]`/`[this+0x20]`,
/// then publishes the index block and three derived pointers through
/// globals. The answer is the block address plus `0x20 + dim1 * 2`.
///
/// Original: 0x00887270 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00887270(this: u32) -> u32 {
    unsafe {
        const DIM0_GLOBAL: u32 = 0x0103_00a8;
        const DIM1_GLOBAL: u32 = 0x0103_00ac;
        const DIM2_GLOBAL: u32 = 0x0103_00b0;
        const BLOCK_GLOBAL: u32 = 0x0115_a45c;
        const P1_GLOBAL: u32 = 0x0115_a458;
        const P0_GLOBAL: u32 = 0x0115_a450;
        const P2_GLOBAL: u32 = 0x0115_a454;
        const ALLOC: u32 = 1;
        let d0 = (lf_checker_rt::relocated(DIM0_GLOBAL) as *const u32)
            .read_unaligned();
        let d1 = (lf_checker_rt::relocated(DIM1_GLOBAL) as *const u32)
            .read_unaligned();
        let d2 = (lf_checker_rt::relocated(DIM2_GLOBAL) as *const u32)
            .read_unaligned();
        ((this + 0x10) as *mut u32).write_unaligned(0);
        ((this + 0x14) as *mut u32).write_unaligned(d0);
        ((this + 0x18) as *mut u32).write_unaligned(d1);
        ((this + 0x1c) as *mut u32).write_unaligned(d2);
        let slots = lf_checker_rt::callee_cdecl!(ALLOC, u32, d0.wrapping_mul(16), 0x10);
        (this as *mut u32).write_unaligned(slots);
        ((this + 0x0c) as *mut u32).write_unaligned(d1.wrapping_sub(d2));
        ((this + 0x20) as *mut u32).write_unaligned(0);
        let block = lf_checker_rt::callee_cdecl!(
            ALLOC, u32, d1.wrapping_mul(4).wrapping_add(0x20), 0x10);
        (lf_checker_rt::relocated(BLOCK_GLOBAL) as *mut u32).write_unaligned(block);
        let p1 = block.wrapping_add(0x10);
        (lf_checker_rt::relocated(P1_GLOBAL) as *mut u32).write_unaligned(p1);
        let p0 = p1.wrapping_add(0x10);
        (lf_checker_rt::relocated(P0_GLOBAL) as *mut u32).write_unaligned(p0);
        let p2 = p0.wrapping_add(d1.wrapping_mul(2));
        (lf_checker_rt::relocated(P2_GLOBAL) as *mut u32).write_unaligned(p2);
        p2
    }
});
