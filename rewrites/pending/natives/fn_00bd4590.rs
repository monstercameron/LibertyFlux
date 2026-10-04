// original: 0x00bd4590 TRIGGER_PTFX_ON_OBJ_BONE
/// `TRIGGER_PTFX_ON_OBJ_BONE` (native hash `0x3A2A77F9`): invoke the shared `Ptfx_Call10Bone`
/// dispatcher with this native's implementation (`NativeImpl_TRIGGER_PTFX_ON_OBJ_BONE`) and the call
/// context. Nothing is written back to the call context.
export!(cdecl, rw_00bd4590(ctx: *const crate::NativeCtx) -> () {
    unsafe {
        callee_cdecl!(1, u32, lf_rn08_rt::relocated(0xbd65c0), ctx as u32);
    }
});
