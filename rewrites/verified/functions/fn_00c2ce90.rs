// original: 0x00c2ce90 register_fixed_object
/// Registers a fixed object with a fixed owner, returns the answer.
export!(cdecl, rw_00c2ce90() -> u32 {
    unsafe { callee_thiscall!(1, u32, relocated(0x16CFD4C), relocated(0xEC6CF4)) }
});
