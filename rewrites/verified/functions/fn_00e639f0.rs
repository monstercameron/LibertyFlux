// original: 0x00e639f0 dyninit_g11d76b8
/// Construct the global object, then register its teardown.
/// The registrar answer passes through in EAX, as in the original.
export!(cdecl, rw_00e639f0() -> u32 {
    callee_thiscall!(0, u32, relocated(0x11D76B8));
    callee_cdecl!(1, u32, relocated(0xE71560))
});
