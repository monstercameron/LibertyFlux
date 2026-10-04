// original: 0x00e62e40 dyninit_g117356c
/// Construct the global object, then register its teardown.
/// The registrar answer passes through in EAX, as in the original.
export!(cdecl, rw_00e62e40() -> u32 {
    callee_thiscall!(0, u32, relocated(0x117356C));
    callee_cdecl!(1, u32, relocated(0xE712A0))
});
