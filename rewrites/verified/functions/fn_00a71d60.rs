// original: 0x00a71d60 resolve_then_copy_pose (proposed)
/// Resolves an inner object through the registry, then fills the two
/// outputs through the pose-copy helper, returning its tag byte.
///
/// `cdecl`, three stack words. Reads the link at `a1+0x224`, adds 0x44,
/// and asks the registry (thiscall, one word: 4) for the inner object.
/// A null answer returns 0 at once. Otherwise calls the pose-copy helper
/// (thiscall on the inner object, `a2` then `a3` -- the first push reads
/// the top slot `a3` and the second push, one slot lower, reads `a2`, so
/// the helper's first output is `a2`) and returns its byte.
lf_checker_rt::export!(cdecl, rw_00a71d60(a1: u32, a2: u32, a3: u32) -> u8 {
    unsafe {
        const LINK_OFF: u32 = 0x224;
        const INNER_BIAS: u32 = 0x44;
        let inner = ((a1 + LINK_OFF) as *const u32)
            .read_unaligned()
            .wrapping_add(INNER_BIAS);
        let t = lf_checker_rt::callee_thiscall!(1, u32, inner, 4);
        if t == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(2, u32, t, a2, a3) as u8
    }
});
