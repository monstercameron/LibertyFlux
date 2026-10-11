// original: 0x005f32d0 rage::phInst::vf2

/// Forwards the incoming source pointer as the callee object and passes a
/// pointer to this instance's `+4` field as its sole stack argument. The helper
/// result is returned unchanged, and the method consumes its one stack word.
lf_checker_rt::export!(thiscall, rw_005f32d0(this: u32, source: u32) -> u32 {
    const FIELD_ARGUMENT: u32 = 0x04;
    lf_checker_rt::callee_thiscall!(1, u32, source, this.wrapping_add(FIELD_ARGUMENT))
});
