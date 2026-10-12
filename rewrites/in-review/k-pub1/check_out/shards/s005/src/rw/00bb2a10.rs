// original: 0x00bb2a10 SET_PLAYER_CAN_USE_COVER
/// Native handler `SET_PLAYER_CAN_USE_COVER` (script context in, engine call out).
///
/// Toggles player cover use; forwards player index and coerced flag.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00bb2a10(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        __lf_callee_cdecl!(1, u32, *a.add(0), u32::from(*a.add(1) != 0))
    }
});

/// Script-native call context, as the handlers observe it (r-n29 shape).
#[repr(C)]
pub struct NativeCtx {
    /// Pointer to the dword return slot.
    pub ret: *mut u32,
    /// Second word; no handler in this batch reads it.
    pub reserved: u32,
    /// Pointer to the argument array (one dword per script argument).
    pub args: *const u32,
}
