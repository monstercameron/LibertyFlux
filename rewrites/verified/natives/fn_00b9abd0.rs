// original: 0x00b9abd0 GET_SORTED_NETWORK_RESTART_NODE_USING_GROUP_LIST
/// GET_SORTED_NETWORK_RESTART_NODE_USING_GROUP_LIST: Returns a sorted network restart node from a group list.
/// Passes the call context plus the engine routine address to the
/// shared native unpacker, which reads the script arguments itself.
lf_rn109_rt::export!(cdecl, rw_fn_00b9abd0(ctx: u32) -> u32 {
    let routine = lf_rn109_rt::relocated(0x00b9cf30);
    lf_rn109_rt::callee_cdecl!(1, u32, routine, ctx,)
});
