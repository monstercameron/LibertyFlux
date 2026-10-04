// original: 0x00b9a620 GET_CLOSEST_NETWORK_RESTART_NODE
/// Script native `GET_CLOSEST_NETWORK_RESTART_NODE` (hash 0x46CD1D73).
///
/// Calls the engine with a node-table address (an immediate in the original
/// with a relocation entry, so it is derived with `relocated()`) and the
/// call context pointer itself. No return slot is written by the handler.
export!(cdecl, rw_00b9a620(ctx: *const u8) -> u32 {
    const NODE_TABLE_FILE_VA: u32 = 0x00B9_B6A0;
    callee_cdecl!(
        1,
        u32,
        lf_k2_rt::relocated(NODE_TABLE_FILE_VA),
        ctx as u32
    )
});
