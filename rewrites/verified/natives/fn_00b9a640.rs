// original: 0x00b9a640 GET_CLOSEST_ROAD
/// Script native `GET_CLOSEST_ROAD` (hash 0x63C00DE7).
///
/// Passes a fixed engine callback address together with the call context to
/// a shared road-search helper. The callback address is a relocated code
/// address (its push-immediate slot carries a HIGHLOW reloc), so it is
/// derived from the loaded image base. No return slot is written.
export!(cdecl, rw_00b9a640(ctx: *const u8) -> u32 {
    callee_cdecl!(1, u32, relocated(0x00B9_B7F0u32), ctx as u32)
});
