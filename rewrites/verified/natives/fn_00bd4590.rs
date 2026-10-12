// original: 0x00bd4590 TRIGGER_PTFX_ON_OBJ_BONE
/// `TRIGGER_PTFX_ON_OBJ_BONE` (native hash `0x3A2A77F9`): invoke the shared `Ptfx_Call10Bone`
/// dispatcher with this native's implementation (`NativeImpl_TRIGGER_PTFX_ON_OBJ_BONE`) and the call
/// context. Nothing is written back to the call context.
export!(cdecl, rw_00bd4590(ctx: *const NativeCtx08) -> () {
    unsafe {
        callee_cdecl!(1, u32, relocated(0xbd65c0), ctx as u32);
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
