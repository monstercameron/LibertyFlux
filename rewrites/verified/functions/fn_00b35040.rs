// original: 0x00B35040 heap_fix_loop_a

/// Sweep `[lo, limit)` in 28-byte steps, calling callee 1 per element with
/// the element copied by value onto the stack.
///
/// Each call passes nine stack words `(p, elem words..., extra)`; the `ecx`
/// the original sets to its copy buffer is dead (the callee never reads
/// `ecx`), so the contract uses plain cdecl. The third incoming word is
/// padding the original never reads; the sweep stops when `p` equals
/// `limit`, so the caller keeps the span a multiple of the stride. Cdecl,
/// four stack words; `eax` on exit is the last callee answer (or entry
/// garbage when the span is empty), so the contract compares no return.
///
/// Original: 0x00B35040.

lf_checker_rt::export!(cdecl, rw_00B35040(lo: u32, limit: u32, _w2: u32, extra: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 28;
        const CALLEE: u32 = 1;
        let mut p = lo;
        while p != limit {
            let b0 = (p as *const u32).read_unaligned();
            let b1 = (p.wrapping_add(4) as *const u32).read_unaligned();
            let b2 = (p.wrapping_add(8) as *const u32).read_unaligned();
            let b3 = (p.wrapping_add(12) as *const u32).read_unaligned();
            let b4 = (p.wrapping_add(16) as *const u32).read_unaligned();
            let b5 = (p.wrapping_add(20) as *const u32).read_unaligned();
            let b6 = (p.wrapping_add(24) as *const u32).read_unaligned();
            let _: u32 =
                lf_checker_rt::callee_cdecl!(CALLEE, u32, p, b0, b1, b2, b3, b4, b5, b6, extra);
            p = p.wrapping_add(STRIDE);
        }
        0
    }
});
