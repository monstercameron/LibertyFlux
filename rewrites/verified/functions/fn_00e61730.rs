// original: 0x00e61730 timing_callback_register_2
/// Forward a fixed handler address to the common registrar.
///
/// Passes the code pointer `0x00E702B0` to the registrar helper (stubbed, cdecl/1). Takes no arguments, ignores entry registers, and returns the registrar's answer.
export!(cdecl, rw_00e61730() -> u32 {
    unsafe {
        callee_cdecl!(0, u32, relocated(0x00e702b0))
    }
});
