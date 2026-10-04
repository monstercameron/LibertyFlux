// original: 0x00b9ab90 GET_SORTED_NETWORK_RESTART_NODE_EXCLUDING_GROUP
/// Script native `GET_SORTED_NETWORK_RESTART_NODE_EXCLUDING_GROUP` (hash 0x55AC75E2).
///
/// The handler forwards its call context together with an engine-function id to a shared unpacker routine (intercepted and scripted by the checker), which reads the script arguments itself. No return slot is written.
export!(cdecl, rw_00b9ab90(ctx: *const u8) -> u32 {
    unsafe {
        // Engine-function id consumed by the shared unpacker (arg0); the
        // call context is arg1. The id is a relocated immediate.
        const ENGINE_ID: u32 = 0x00B9CD90;
        callee_cdecl!(1, u32, relocated(ENGINE_ID), ctx as u32)
    }
});
