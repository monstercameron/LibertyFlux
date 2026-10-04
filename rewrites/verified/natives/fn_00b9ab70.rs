// original: 0x00b9ab70 GET_SORTED_NETWORK_RESTART_NODE
/// Find a sorted network restart node (unused by shipped scripts).
///
/// Hands the call context and this native's per-native callback address to the
/// shared restart-node dispatcher. The callback address is derived with
/// `relocated`, never hard-coded. Returns whatever the dispatcher returned.
export!(cdecl, rw_00b9ab70(ctx: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0x00B9CCB0), ctx) }
});
