// original: 0x00e600e0 init_and_register_teardown_00e6f700
/// Run a module initializer, then register its teardown callback.
///
/// Returns the registration result.
export!(cdecl, rw_00e600e0() -> u32 {
    unsafe {
        /// Teardown callback installed by this initializer.
        const TEARDOWN: u32 = 0x00E6F700;
        callee_cdecl!(1, u32,);
        callee_cdecl!(2, u32, relocated(TEARDOWN))
    }
});
