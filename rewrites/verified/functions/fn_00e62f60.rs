// original: 0x00e62f60 dyninit_g11737b0
/// Construct the global object, then register its teardown.
/// The registrar answer passes through in EAX, as in the original.
export!(cdecl, rw_00e62f60() -> u32 {
    callee_thiscall!(0, u32, relocated(0x11737B0));
    callee_cdecl!(1, u32, relocated(0xE712D0))
});
