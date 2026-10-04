// original: 0x00e61200 forward_ptr_e6fff0
/// Forward the fixed descriptor at 0x00E6FFF0 to the shared helper.
///
/// Same shape as [`rw_00e60f20`]: one constant, one call, answer in EAX.
export!(cdecl, rw_00e61200() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0x00E6FFF0)) }
});
