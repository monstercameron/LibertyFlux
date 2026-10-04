// original: 0x00e63020 dyninit_g1176d48
/// Construct the global object, then register its teardown.
/// The registrar answer passes through in EAX, as in the original.
export!(cdecl, rw_00e63020() -> u32 {
    callee_thiscall!(0, u32, relocated(0x1176D48));
    callee_cdecl!(1, u32, relocated(0xE71320))
});
