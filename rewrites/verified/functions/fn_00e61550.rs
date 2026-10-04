// original: 0x00e61550 timing_state_reset_and_register
/// Reset the shared timing state object, then register its callback.
///
/// Passes the state object address to the reset routine, then returns the
/// registrar's answer.
export!(cdecl, rw_00e61550() -> u32 {
    unsafe {
        callee_thiscall!(1, u32, relocated(0x01A00E60));
        callee_cdecl!(2, u32, relocated(0x00E701A0))
    }
});
