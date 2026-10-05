// original: 0x00605400 euphoria_this_plus16_thunk (proposed)

/// Forward with a shifted receiver: add `THIS_BIAS` to `this` and tail-call
/// the target with it and the two forwarded stack words `a0` and `a1`.
///
/// `this` points into a larger object and the real method runs on the
/// sub-object 16 bytes in (a base-class adjustment). `a0` is the target's
/// object argument and `a1` a second word, both forwarded opaquely (the
/// target cleans them: thiscall, two arguments). The target's 32-bit result
/// is returned unchanged.
///
/// Original: 0x00605400 (thiscall; `(an instruction of the original)` then a jump).
lf_checker_rt::export!(thiscall, rw_00605400(this: u32, a0: u32, a1: u32) -> u32 {
    const THIS_BIAS: u32 = 0x10;
    const TARGET: u32 = 1;
    unsafe {
        let adj = this.wrapping_add(THIS_BIAS);
        lf_checker_rt::callee_thiscall!(TARGET, u32, adj, a0, a1)
    }
});
