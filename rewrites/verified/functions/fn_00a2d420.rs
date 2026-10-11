// original: 0x00a2d420 CPlayerPed::vf78

/// Store the supplied 32-bit value in the player ped's field at `+0xeac`
/// and return the same bits in EAX. The method takes `this` in ECX, one
/// stack argument, and the original callee removes that argument.
lf_checker_rt::export!(thiscall, rw_00a2d420(this: u32, value: u32) -> u32 {
    const FIELD_EAC: u32 = 0x0eac;
    unsafe { ((this + FIELD_EAC) as *mut u32).write_unaligned(value) };
    value
});
