// original: 0x00b765c0 forward_and_return_this (proposed)

/// Forward `this` to the callee at 0xb76ac0 (thiscall, no stack words) and
/// return `this` unchanged. The callee's answer is ignored.
///
/// Original: 0x00b765c0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00b765c0(this: u32) -> u32 {
    const FWD_CALLEE: u32 = 1;
    let _: u32 = lf_checker_rt::callee_thiscall!(FWD_CALLEE, u32, this);
    this
});
