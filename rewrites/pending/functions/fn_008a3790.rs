// original: 0x008a3790 forward_cdecl3_to_thiscall2_8a3230
/// Forwards three cdecl arguments to a two-argument `thiscall` callee.
///
/// The first argument becomes the `this` pointer; the other two are passed on
/// the stack. Returns the callee's answer unchanged.
export!(cdecl, rw_008a3790(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe { callee_thiscall!(1, u32, a0, a1, a2) }
});
