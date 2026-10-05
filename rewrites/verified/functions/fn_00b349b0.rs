// original: 0x00B349B0 heap_loop_16

/// 16-byte-element twin of `rw_00B34890`.
///
/// `count = (end - base) >> 4` (arithmetic shift; exact on the aligned,
/// non-negative spans the caller keeps). Same index loop, with eight-word
/// calls `(base, idx, count, elem words..., extra)`. The `eax` the original
/// points at its copy buffer is dead (a sibling site calls the same callee
/// with a small integer in `eax`), so the contract is plain cdecl. Cdecl,
/// three stack words, no meaningful return value.
///
/// Original: 0x00B349B0.

lf_checker_rt::export!(cdecl, rw_00B349B0(base: u32, end: u32, extra: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 16;
        const CALLEE: u32 = 1;
        let count = (end.wrapping_sub(base) as i32) >> 4;
        if count >= 2 {
            let mut idx = (count - 2) / 2;
            loop {
                let src = base.wrapping_add((idx as u32).wrapping_mul(STRIDE));
                let b0 = (src as *const u32).read_unaligned();
                let b1 = (src.wrapping_add(4) as *const u32).read_unaligned();
                let b2 = (src.wrapping_add(8) as *const u32).read_unaligned();
                let b3 = (src.wrapping_add(12) as *const u32).read_unaligned();
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    CALLEE, u32, base, idx as u32, count as u32, b0, b1, b2, b3, extra
                );
                if idx == 0 {
                    break;
                }
                idx -= 1;
            }
        }
        0
    }
});
