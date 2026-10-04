// original: 0x00bc6060 GET_OFFSETS_FOR_ATTACH_CAR_TO_CAR
/// Script native `GET_OFFSETS_FOR_ATTACH_CAR_TO_CAR` (hash 0x2CAD4E39).
///
/// Passes an engine descriptor address (0xBCB490, relocated at load) plus
/// the call context itself to a shared attach-offsets worker. No return
/// slot is written by the handler itself.
export!(cdecl, rw_00bc6060(ctx: *const u8) -> u32 {
    callee_cdecl!(1, u32, relocated(0xBCB490), ctx as u32)
});
