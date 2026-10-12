// original: 0x00bd8ea0 NETWORK_STRING_VERIFY_PENDING
/// Native handler `NETWORK_STRING_VERIFY_PENDING` (script context in, engine call out).
///
/// True while a network string verification is pending; zero-arg query, stores the byte answer.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00bd8ea0(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        let r = __lf_callee_cdecl!(1, u32, );
        // The original keeps only AL and zero-extends it into the slot.
        *(*ctx).ret = r & 0xFF;
        (*ctx).ret as u32
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
