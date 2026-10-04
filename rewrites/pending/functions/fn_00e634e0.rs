// original: 0x00e634e0 dyninit_g119bfa8
/// Construct the global object, then register its teardown.
/// The registrar answer passes through in EAX, as in the original.
export!(cdecl, rw_00e634e0() -> u32 {
    callee_thiscall!(0, u32, relocated(0x119BFA8));
    callee_cdecl!(1, u32, relocated(0xE71430))
});
