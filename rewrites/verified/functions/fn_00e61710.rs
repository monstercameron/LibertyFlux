// original: 0x00e61710 timing_state_reset_and_register
/// Clear two timer state words, then register the handler.
///
/// Writes zero to the dword at `0x01A0224C` and the word at `0x01A02250`, then passes the code pointer `0x00E70280` to the registrar helper (stubbed, cdecl/1) and returns its answer.
export!(cdecl, rw_00e61710() -> u32 {
    unsafe {
        *global::<u32>(0x01a0224c) = 0;
        *global::<u16>(0x01a02250) = 0;
        callee_cdecl!(0, u32, relocated(0x00e70280))
    }
});
