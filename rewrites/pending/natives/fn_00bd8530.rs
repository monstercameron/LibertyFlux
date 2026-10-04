// original: 0x00BD8530 NETWORK_GET_TEAM_OPTION
// Rewrite of the NETWORK_GET_TEAM_OPTION native handler.

/// Script native `NETWORK_GET_TEAM_OPTION()`.
///
/// Takes no script arguments: calls the engine team-option routine and stores
/// the zero-extended low byte of its answer in the context's return slot.
export!(cdecl, rw_bd8530(ctx: *const u8) -> u32 {
    unsafe {
        let answer: u32 = callee_cdecl!(1, u32,);
        let ret_slot = *(ctx as *const *mut u32);
        *ret_slot = answer & 0xFF;
        0
    }
});
