// original: 0x00e62410 net_regfwd_17
/// Forward one code pointer to the registrar helper and return its answer.
///
/// Pushes `0x00E70C20` and calls the registrar (stubbed, cdecl/1), which the
/// caller cleans up. Takes no arguments; entry registers are ignored.
/// Returns the registrar answer unchanged.
export!(cdecl, rw_00e62410() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0xE70C20)) }
});
