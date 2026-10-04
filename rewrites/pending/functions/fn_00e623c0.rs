// original: 0x00e623c0 net_regfwd_12
/// Forward one code pointer to the registrar helper and return its answer.
///
/// Pushes `0x00E70B90` and calls the registrar (stubbed, cdecl/1), which the
/// caller cleans up. Takes no arguments; entry registers are ignored.
/// Returns the registrar answer unchanged.
export!(cdecl, rw_00e623c0() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0xE70B90)) }
});
