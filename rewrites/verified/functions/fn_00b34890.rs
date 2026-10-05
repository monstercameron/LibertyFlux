// original: 0x00B34890 heap_loop_28_a

/// Drive callee 1 over the lower half of a 28-byte-element heap range.
///
/// `count = (end - base) / 28` (C truncation). For `idx` from `(count-2)/2`
/// down to 0 (no calls when `count < 2`), calls callee 1 with eleven stack
/// words `(base, idx, count, elem words..., extra)`: the element travels by
/// value on the stack. The `ecx` the original points at its own copy buffer
/// is dead (a sibling site calls the same callee with a small integer in
/// `ecx`, which would fault if dereferenced), so the contract is plain
/// cdecl and compares every word. Cdecl, three stack words; `eax` on exit
/// is the last loaded word (or the division remainder when no call runs),
/// which no caller can use, so the contract compares no return value.
///
/// Original: 0x00B34890.

lf_checker_rt::export!(cdecl, rw_00B34890(base: u32, end: u32, extra: u32) -> u32 {
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
