// original: 0x00bd83b0 NETWORK_GET_HOST_LATENCY
/// `NETWORK_GET_HOST_LATENCY` (native hash `0x74093768`): forward 1 script argument to the
/// `NativeImpl_NETWORK_GET_HOST_LATENCY` engine function and store its full 32-bit result
/// into the context return slot.
export!(cdecl, rw_00bd83b0(ctx: *const NativeCtx08) -> () {
    unsafe {
        let args = (*ctx).args;
        let answer: u32 = callee_cdecl!(1, u32, *args);
        *(*ctx).ret_slot = answer;
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
