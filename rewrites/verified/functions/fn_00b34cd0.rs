// original: 0x00B34CD0 heap_push_one_28_a

/// Copy one 28-byte element from `src` to `dst`, then drive callee 1 once
/// with the seven argument words `w3..=w9` as its struct by value.
///
/// `count = (end - src) / 28` (C truncation). Callee 1 runs with eleven
/// stack words `(src, 0, count, w3..=w9, w10)`; the `ecx` the original
/// briefly points at its copy buffer is dead (it holds `end - src` at the
/// call), so the contract is plain cdecl and compares every word. Returns
/// the callee's answer. Cdecl, eleven stack words.
///
/// Original: 0x00B34CD0.

lf_checker_rt::export!(
    cdecl,
    rw_00B34CD0(
        src: u32, end: u32, dst: u32, w3: u32, w4: u32, w5: u32, w6: u32,
        w7: u32, w8: u32, w9: u32, w10: u32
    ) -> u32 {
        unsafe {
            const CALLEE: u32 = 1;
            for i in 0..7u32 {
                let w = (src.wrapping_add(i * 4) as *const u32).read_unaligned();
                (dst.wrapping_add(i * 4) as *mut u32).write_unaligned(w);
            }
            let count = (end.wrapping_sub(src) as i32) / 28;
            lf_checker_rt::callee_cdecl!(
                CALLEE, u32, src, 0, count as u32, w3, w4, w5, w6, w7, w8, w9, w10
            )
        }
    }
);
