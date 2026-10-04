// original: 0x00b9aa70 GET_RANDOM_NETWORK_RESTART_NODE
/// Script native `GET_RANDOM_NETWORK_RESTART_NODE` (hash 0x0A2B76C2).
///
/// Forwards the call context together with a fixed engine address to
/// the engine worker. No return slot is written by the handler itself.
export!(cdecl, rw_00b9aa70(ctx: *const u8) -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0xB9C850), ctx as u32) }
});
