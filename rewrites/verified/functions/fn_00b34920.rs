// original: 0x00B34920 heap_loop_28_b

/// Twin of `rw_00B34890` driving the neighbouring callee.
///
/// Identical loop and eleven-word call shape. Cdecl, three stack words, no
/// meaningful return value.
///
/// Original: 0x00B34920.

lf_checker_rt::export!(cdecl, rw_00B34920(base: u32, end: u32, extra: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 28;
        const CALLEE: u32 = 1;
        let count = (end.wrapping_sub(base) as i32) / 28;
        if count >= 2 {
            let mut idx = (count - 2) / 2;
            loop {
                let src = base.wrapping_add((idx as u32).wrapping_mul(STRIDE));
                let b0 = (src as *const u32).read_unaligned();
                let b1 = (src.wrapping_add(4) as *const u32).read_unaligned();
                let b2 = (src.wrapping_add(8) as *const u32).read_unaligned();
                let b3 = (src.wrapping_add(12) as *const u32).read_unaligned();
                let b4 = (src.wrapping_add(16) as *const u32).read_unaligned();
                let b5 = (src.wrapping_add(20) as *const u32).read_unaligned();
                let b6 = (src.wrapping_add(24) as *const u32).read_unaligned();
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    CALLEE, u32, base, idx as u32, count as u32,
                    b0, b1, b2, b3, b4, b5, b6, extra
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
