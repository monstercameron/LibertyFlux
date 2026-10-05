// original: 0x009441a0 streaming_pair_init (proposed)

/// Initialise the two streaming lane records back to back.
///
/// Calls the lane initialiser (thiscall, no stack arguments) first with
/// `this`, then with `this + 0xbd0`. Returns the second call's answer.
///
/// Original: 0x009441a0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_009441a0(this: u32) -> u32 {
    const STRIDE: u32 = 0xBD0;
    const CALLEE: u32 = 1;
    let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE, u32, this);
    lf_checker_rt::callee_thiscall!(CALLEE, u32, this.wrapping_add(STRIDE))
});
