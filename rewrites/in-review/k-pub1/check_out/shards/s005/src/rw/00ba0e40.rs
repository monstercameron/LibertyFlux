// original: 0x00ba0e40 SET_ANIM_GROUP_FOR_CHAR
/// Native handler `SET_ANIM_GROUP_FOR_CHAR` (script context in, engine call out).
///
/// Sets the char's animation group; forwards ped handle and group-name pointer.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00ba0e40(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        __lf_callee_cdecl!(1, u32, *a.add(0), *a.add(1))
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
