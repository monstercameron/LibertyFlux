// original: 0x00b34cd0 sort_sift_step_key_c (proposed)

/// Copy the 28-byte element at `src` over the element at `dst`, then call
/// the keyed sift routine with `src`, a zero flag, the element count
/// `(limit - src) / 28` (signed), the 28-byte `value` passed by value, and
/// the context word. The trailing argument is unread. Returns the sift
/// call's answer. Original: 0x00b34cd0 (cdecl, twelve stack words: src,
/// limit, dst, seven value words, context, unused).
lf_checker_rt::export!(cdecl, rw_00b34cd0(
    src: u32,
    limit: u32,
    dst: u32,
    v0: u32,
    v1: u32,
    v2: u32,
    v3: u32,
    v4: u32,
    v5: u32,
    v6: u32,
    ctx: u32,
    _unused: u32,
) -> u32 {
    unsafe {
        const SIFT: u32 = 1;
        const ELEM: u32 = 28;
        const N_COPY: usize = 7;
        core::ptr::copy_nonoverlapping(
            src as *const u32,
            dst as *mut u32,
            N_COPY,
        );
        let count =
            (limit.wrapping_sub(src) as i32 / ELEM as i32) as u32;
        lf_checker_rt::callee_cdecl!(
            SIFT, u32, src, 0, count, v0, v1, v2, v3, v4, v5, v6, ctx
        )
    }
});
