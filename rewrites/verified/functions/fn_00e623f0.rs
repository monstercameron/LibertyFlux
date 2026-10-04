// original: 0x00e623f0 net_regfwd_15
/// Forward one code pointer to the registrar helper and return its answer.
///
/// Pushes `0x00E70C00` and calls the registrar (stubbed, cdecl/1), which the
/// caller cleans up. Takes no arguments; entry registers are ignored.
/// Returns the registrar answer unchanged.
export!(cdecl, rw_00e623f0() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0xE70C00)) }
});
