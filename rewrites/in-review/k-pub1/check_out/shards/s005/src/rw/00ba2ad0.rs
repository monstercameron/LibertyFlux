// original: 0x00ba2ad0 WARP_CHAR_FROM_CAR_TO_COORD
/// Native handler `WARP_CHAR_FROM_CAR_TO_COORD` (script context in, engine call out).
///
/// Pulls the char out of the car at a coord; forwards handle and three floats.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00ba2ad0(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        __lf_callee_cdecl!(1, u32, *a.add(0), *a.add(1), *a.add(2), *a.add(3))
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
