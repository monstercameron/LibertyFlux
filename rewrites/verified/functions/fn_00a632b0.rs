// original: 0x00A632B0 NativeImpl_IS_CHAR_IN_MELEE_COMBAT

/// Forward this intelligence context and native identifier `0x1af` to the
/// native bridge and return its EAX result. The outer `this` is in ECX; the
/// bridge receives `this + 0x44` in ECX and the identifier as one stack word.
lf_checker_rt::export!(thiscall, rw_00a632b0(this: u32) -> u32 {
    const NATIVE_ID: u32 = 0x1af;
    lf_checker_rt::callee_thiscall!(1, u32, this.wrapping_add(0x44), NATIVE_ID)
});
