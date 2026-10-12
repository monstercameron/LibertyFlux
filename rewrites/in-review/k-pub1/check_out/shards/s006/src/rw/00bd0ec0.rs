// original: 0x00bd0ec0 GET_IS_STICKY_BOMB_STUCK_TO_OBJECT
/// `GET_IS_STICKY_BOMB_STUCK_TO_OBJECT` (native hash `0x04D623FF`): forward 1 script argument to the
/// `NativeImpl_GET_IS_STICKY_BOMB_STUCK_TO_OBJECT` engine function and store the low byte of its result
/// (zero-extended) into the context return slot.
export!(cdecl, rw_00bd0ec0(ctx: *const NativeCtx08) -> () {
    unsafe {
        let args = (*ctx).args;
        let answer: u32 = __lf_callee_cdecl!(1, u32, *args);
        *(*ctx).ret_slot = answer & 0xFF;
    }
});

/// Native call context, r-n08 shape: return-slot pointer, reserved word,
/// argument-array pointer, argument count. (Renamed from r-n08's NativeCtx.)
#[repr(C)]
pub struct NativeCtx08 {
    pub ret_slot: *mut u32,
    pub reserved: u32,
    pub args: *const u32,
    pub argc: u32,
}
