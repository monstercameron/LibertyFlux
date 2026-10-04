// original: 0x00e632c0 dyninit_g118d7f0
/// Construct the global object, then register its teardown.
/// The registrar answer passes through in EAX, as in the original.
export!(cdecl, rw_00e632c0() -> u32 {
    callee_thiscall!(0, u32, relocated(0x118D7F0));
    callee_cdecl!(1, u32, relocated(0xE71370))
});
