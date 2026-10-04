// original: 0x00bd83b0 NETWORK_GET_HOST_LATENCY
/// `NETWORK_GET_HOST_LATENCY` (native hash `0x74093768`): forward 1 script argument to the
/// `NativeImpl_NETWORK_GET_HOST_LATENCY` engine function and store its full 32-bit result
/// into the context return slot.
export!(cdecl, rw_00bd83b0(ctx: *const crate::NativeCtx08) -> () {
    unsafe {
        let args = (*ctx).args;
        let answer: u32 = callee_cdecl!(1, u32, *args);
        *(*ctx).ret_slot = answer;
    }
});
