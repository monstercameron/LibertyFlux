// original: 0x00b9abb0 GET_SORTED_NETWORK_RESTART_NODE_OF_GROUP

/// Native handler `GET_SORTED_NETWORK_RESTART_NODE_OF_GROUP`.
///
/// Forward the whole call context plus a sort helper address to the network restart node routine.
/// Forwards 2 argument(s) to the engine and returns nothing.
export!(cdecl, rw_00b9abb0(ctx: *const u32) -> u32 {
    unsafe {
        callee_cdecl!(1, u32, relocated(0xB9CE70), ctx as u32);
        0
    }
});
