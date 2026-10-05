// original: 0x00B350E0 heap_fix_loop_16

/// 16-byte-element twin of `rw_00B35040` with six-word calls.
///
/// Each call passes `(p, elem words..., extra)`; the `eax` the original
/// holds its copy buffer in is dead (the callee never reads `eax`), so the
/// contract uses plain cdecl. Cdecl, four stack words (one padding), no
/// meaningful return value.
///
/// Original: 0x00B350E0.

lf_checker_rt::export!(cdecl, rw_00B350E0(lo: u32, limit: u32, _w2: u32, extra: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 16;
        const CALLEE: u32 = 1;
        let mut p = lo;
        while p != limit {
            let b0 = (p as *const u32).read_unaligned();
            let b1 = (p.wrapping_add(4) as *const u32).read_unaligned();
            let b2 = (p.wrapping_add(8) as *const u32).read_unaligned();
            let b3 = (p.wrapping_add(12) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_cdecl!(CALLEE, u32, p, b0, b1, b2, b3, extra);
            p = p.wrapping_add(STRIDE);
        }
        0
    }
});
