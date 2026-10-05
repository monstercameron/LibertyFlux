// original: 0x006053C0 euphoria_inner_this_thunk (proposed)

/// Forward to the inner object: load the receiver at `this + INNER_OFF` and
/// tail-call the target with it and the one forwarded stack word `a0`.
///
/// `this` points to an outer Euphoria object whose word at `+0x2f0` holds the
/// inner receiver the real method runs on. The stack word is forwarded
/// opaquely (the target cleans it: thiscall, one argument). The target's
/// 32-bit result is returned unchanged.
///
/// Original: 0x006053C0 (thiscall; `(an instruction of the original)` then a jump).
lf_checker_rt::export!(thiscall, rw_006053C0(this: u32, a0: u32) -> u32 {
    const INNER_OFF: u32 = 0x2f0;
    const TARGET: u32 = 1;
    unsafe {
        let inner = ((this.wrapping_add(INNER_OFF)) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(TARGET, u32, inner, a0)
    }
});
