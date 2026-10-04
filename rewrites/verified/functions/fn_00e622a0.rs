// original: 0x00e622a0 net_regfwd_01
/// Forward one code pointer to the registrar helper and return its answer.
///
/// Pushes `0x00E709E0` and calls the registrar (stubbed, cdecl/1), which the
/// caller cleans up. Takes no arguments; entry registers are ignored.
/// Returns the registrar answer unchanged.
export!(cdecl, rw_00e622a0() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0xE709E0)) }
});
