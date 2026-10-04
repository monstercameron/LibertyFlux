// original: 0x00e61130 forward_ptr_e6ff00
/// Forward the fixed descriptor at 0x00E6FF00 to the shared helper.
///
/// Same shape as [`rw_00e60f20`]: one constant, one call, answer in EAX.
export!(cdecl, rw_00e61130() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0x00E6FF00)) }
});
