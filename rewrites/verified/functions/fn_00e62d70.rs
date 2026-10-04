// original: 0x00e62d70 dyninit_g116bff0
/// Construct the global object, then register its teardown.
/// The registrar answer passes through in EAX, as in the original.
export!(cdecl, rw_00e62d70() -> u32 {
    callee_thiscall!(0, u32, relocated(0x116BFF0));
    callee_cdecl!(1, u32, relocated(0xE71290))
});
