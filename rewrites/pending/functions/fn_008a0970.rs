// original: 0x008a0970 forward_cdecl3_to_thiscall2_8a0150
/// Forwards three cdecl arguments to a two-argument `thiscall` callee.
///
/// The first argument becomes the `this` pointer; the other two are passed on
/// the stack. Returns the callee's answer unchanged.
export!(cdecl, rw_008a0970(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe { callee_thiscall!(1, u32, a0, a1, a2) }
});
