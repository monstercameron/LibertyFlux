// original: 0x00e5fe60 register_teardown_00e6f560
/// Register a fixed teardown callback.
///
/// Forwards the callback address to the shared registrar and returns
/// its result.
export!(cdecl, rw_00e5fe60() -> u32 {
    unsafe {
        /// Teardown callback to register.
        const TEARDOWN: u32 = 0x00E6F560;
        callee_cdecl!(1, u32, relocated(TEARDOWN))
    }
});
