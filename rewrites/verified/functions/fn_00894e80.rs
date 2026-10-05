// original: 0x00894e80 this_adjust_forward_550 (proposed)

/// Forward to the worker routine with the object pointer adjusted.
///
/// Adds `0x550` to `this` and tail-calls the target, passing `a0` through
/// and returning its result. The original is an 11-byte thunk (add then
/// jump); the rewrite expresses it as a call so the intercepted callee can
/// answer. The target takes one stack word (its body ends the callee pops 4 bytes).
///
/// Original: 0x00894e80 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00894e80(this: u32, a0: u32) -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, this.wrapping_add(0x550), a0)
});
