// original: 0x00a91e10 stream_vector_append

/// Appends a pointer-sized slot to the global dword vector.
///
/// The global at file VA 0x12FB274 holds a count at `[G]` with elements at
/// `[G+4], ...`. When `n` is 0 the stored element is 0 with no call;
/// otherwise the callee (callee 1, cdecl, allocator) is called with
/// `n * 4`, or 0xFFFFFFFF when that multiply overflows, and its answer is
/// stored. Either way the count is bumped and the previous count (the new
/// slot's index) is returned. One call on the non-zero path.
/// Original: 0x00A91E10 (cdecl, one stack word), 81 bytes.
lf_checker_rt::export!(cdecl, rw_00a91e10(n: u32) -> u32 {
    unsafe {
        const VEC: u32 = 0x12FB274;
        const ALLOC: u32 = 1;
        let elem: u32 = if n == 0 {
            0
        } else {
            let (bytes, overflow) = n.overflowing_mul(4);
            let size = if overflow { 0xFFFF_FFFF } else { bytes };
            lf_checker_rt::callee_cdecl!(ALLOC, u32, size)
        };
        let g = lf_checker_rt::global::<u32>(VEC);
        let count = g.read();
        let next = count.wrapping_add(1);
        g.offset(next as isize).write_unaligned(elem);
        g.write_unaligned(next);
        next.wrapping_sub(1)
    }
});
