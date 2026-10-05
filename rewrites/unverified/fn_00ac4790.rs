// original: 0x00AC4790 copy_vec4_and_broadcast_w (proposed)

/// Copy four words from `src` to `dst`, then broadcast `src[3]` to `out`.
///
/// The original copies words 0..4 in order, re-reads `src[3]` after the copy,
/// and stores it to all four words of `out` (cdecl, three pointers). The
/// re-read is kept so overlapping `src`/`dst` behave identically.
lf_checker_rt::export!(cdecl, rw_00AC4790(src: u32, dst: u32, out: u32) -> u32 {
    unsafe {
        for i in 0..4u32 {
            let w = (src.wrapping_add(i * 4) as *const u32).read_unaligned();
            (dst.wrapping_add(i * 4) as *mut u32).write_unaligned(w);
        }
        let w3 = (src.wrapping_add(12) as *const u32).read_unaligned();
        for i in 0..4u32 {
            (out.wrapping_add(i * 4) as *mut u32).write_unaligned(w3);
        }
        0
    }
});
