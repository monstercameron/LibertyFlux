// original: 0x00b9a8f0 GET_NTH_CLOSEST_CAR_NODE_FAVOUR_DIRECTION
/// Script native `GET_NTH_CLOSEST_CAR_NODE_FAVOUR_DIRECTION`.
///
/// Passes a constant identifying the direction-favouring search worker plus
/// the call context itself to a shared engine helper, which reads the
/// script arguments out of the context. No return slot is written by the
/// handler itself. The constant is a relocated image address, resolved here
/// through the checker's base table like the original's fixup does.
export!(cdecl, rw_00b9a8f0(ctx: *const u8) -> u32 {
    const SEARCH_WORKER: u32 = 0x00B9_C120;
    callee_cdecl!(1, u32, relocated(SEARCH_WORKER), ctx as u32)
});
