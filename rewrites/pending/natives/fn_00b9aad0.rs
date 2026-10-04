// original: 0x00b9aad0 GET_RANDOM_NETWORK_RESTART_NODE_USING_GROUP_LIST
lf_rn22_rt::export!(cdecl,
    /// Script native `GET_RANDOM_NETWORK_RESTART_NODE_USING_GROUP_LIST`.
    /// Passes a fixed engine helper address and the call context to the
    /// network engine function, which fills in the restart node. No script
    /// return value at handler level.
    rw_00b9aad0(ctx: u32) -> u32 {
    // The pushed helper address is position-dependent in the original.
    lf_rn22_rt::callee_cdecl!(1, u32, lf_rn22_rt::relocated(0x00B9CAB0), ctx)
});
