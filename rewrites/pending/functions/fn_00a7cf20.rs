// original: 0x00a7cf20 forward_shifted_tail_call
/// Shifts both the object and the argument past the embedded header and tail
/// calls into the next module. (The original ends in a jump; the rewrite
/// forwards through the checker's intercepted callee instead.)
export!(thiscall, rw_00a7cf20(obj: u32, arg: u32) -> u32 {
    callee_thiscall!(1, u32, obj.wrapping_add(0x10), arg.wrapping_add(0x10))
});
