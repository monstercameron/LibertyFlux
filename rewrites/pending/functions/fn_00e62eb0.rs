// original: 0x00e62eb0 dyninit_g11736fc
/// Construct the global object, then register its teardown.
/// The registrar answer passes through in EAX, as in the original.
export!(cdecl, rw_00e62eb0() -> u32 {
    callee_thiscall!(0, u32, relocated(0x11736FC));
    callee_cdecl!(1, u32, relocated(0xE712B0))
});
