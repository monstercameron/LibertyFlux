// original: 0x00e63a20 dyninit_g11d7678
/// Construct the global object, then register its teardown.
/// The registrar answer passes through in EAX, as in the original.
export!(cdecl, rw_00e63a20() -> u32 {
    callee_thiscall!(0, u32, relocated(0x11D7678));
    callee_cdecl!(1, u32, relocated(0xE71590))
});
