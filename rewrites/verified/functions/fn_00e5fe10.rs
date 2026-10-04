// original: 0x00e5fe10 init_and_register_teardown_00e6f520
/// Run a module initializer, then register its teardown callback.
///
/// Returns the registration result.
export!(cdecl, rw_00e5fe10() -> u32 {
    unsafe {
        /// Teardown callback installed by this initializer.
        const TEARDOWN: u32 = 0x00E6F520;
        callee_cdecl!(1, u32,);
        callee_cdecl!(2, u32, relocated(TEARDOWN))
    }
});
