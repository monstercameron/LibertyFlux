// original: 0x00b9aaf0 GET_RANDOM_WATER_NODE
/// Script native `GET_RANDOM_WATER_NODE` (hash 0x6FBE6CE6).
///
/// Passes the call context itself plus a fixed engine callback address
/// (derived with `relocated`, never hard-coded) to the engine search
/// routine. No return slot is written. No script argument words are read.
export!(cdecl, rw_00b9aaf0(ctx: *const u8) -> u32 {
    unsafe {
        callee_cdecl!(1, u32, relocated(WATER_NODE_CALLBACK), ctx as u32)
    }
});
