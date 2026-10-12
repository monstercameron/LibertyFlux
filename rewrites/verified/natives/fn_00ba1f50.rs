// original: 0x00ba1f50 SET_DECISION_MAKER_ATTRIBUTE_TARGET_LOSS_RESPONSE
/// Native handler `SET_DECISION_MAKER_ATTRIBUTE_TARGET_LOSS_RESPONSE` (script context in, engine call out).
///
/// Sets a decision-maker attribute; forwards two integers.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00ba1f50(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        callee_cdecl!(1, u32, *a.add(0), *a.add(1))
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
