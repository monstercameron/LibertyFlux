// original: 0x00e5ff70 callback_forwarder_e6f600
/// Forwards one fixed callback address to the shared mainloop helper.
///
/// The original pushes the relocated address `0x00e6f600` and calls the helper
/// (caller cleanup, so cdecl/1); the helper's answer is returned unchanged.
/// The helper is intercepted by the checker, so only the call and its
/// argument are observed.
export!(cdecl, rw_00e5ff70() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0x00E6F600)) }
});
