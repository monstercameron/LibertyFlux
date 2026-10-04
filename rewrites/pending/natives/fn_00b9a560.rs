// original: 0x00b9a560 GET_CLOSEST_CAR_NODE_FAVOUR_DIRECTION
/// Script native `GET_CLOSEST_CAR_NODE_FAVOUR_DIRECTION` (hash 0x2F2405D1).
///
/// Calls the engine worker with a relocated image address and the call
/// context pointer itself. This handler reads no script arguments: the
/// address selects the shared node-search routine (Inferred) and the
/// context gives it the script arguments. No return slot is written.
export!(cdecl, rw_00b9a560(ctx: *const u8) -> u32 {
    unsafe {
        // The pushed word is a relocated image address (Verified: a fixup
        // entry covers the push-immediate offset), so it is derived with
        // relocated(), never hard-coded.
        callee_cdecl!(1, u32, relocated(0x00B9B350), ctx as u32)
    }
});
