// original: 0x00b9e040 ATTACH_PED_TO_CAR2
/// Script native `ATTACH_PED_TO_CAR2` (hash 0x10C91E4D).
///
/// Passes the engine a fixed helper address together with the script call
/// context itself, and returns the engine answer. The helper address is an
/// opaque code address the handler forwards as data (never invoked under the
/// checker, which stubs the callee); it is a relocated reference, derived
/// with `relocated()` like any other original address.
export!(cdecl, rw_00b9e040(ctx: *const u8) -> u32 {
    callee_cdecl!(1, u32, relocated(0x00BA4180), ctx as u32)
});
