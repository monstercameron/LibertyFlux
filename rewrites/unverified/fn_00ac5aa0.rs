// original: 0x00AC5AA0 copy_vec3_and_set_w (proposed)

/// Copy three words from `src` to `dst`, then store `w` as the fourth.
///
/// The original copies words 0..3 in order and writes the float argument to
/// `dst[3]` (cdecl: dst, src, float-as-word). No value is returned.
lf_checker_rt::export!(cdecl, rw_00AC5AA0(dst: u32, src: u32, w: u32) -> u32 {
    unsafe {
        for i in 0..3u32 {
            let v = (src.wrapping_add(i * 4) as *const u32).read_unaligned();
            (dst.wrapping_add(i * 4) as *mut u32).write_unaligned(v);
        }
        (dst.wrapping_add(12) as *mut u32).write_unaligned(w);
        0
    }
});
