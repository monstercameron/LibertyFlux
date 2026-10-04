// original: 0x008a09a0 forward_cdecl2_to_thiscall1_8a06f0
/// Forwards two cdecl arguments to the slot-lookup routine as `thiscall`.
///
/// The first argument becomes the `this` pointer; the second is passed on the
/// stack. Returns the callee's answer unchanged.
export!(cdecl, rw_008a09a0(a0: u32, a1: u32) -> u32 {
    unsafe { callee_thiscall!(1, u32, a0, a1) }
});
