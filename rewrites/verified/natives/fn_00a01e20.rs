// original: 0x00a01e20 SET_OBJECT_PHYSICS_PARAMS
/// Script native `SET_OBJECT_PHYSICS_PARAMS` (hash 0x1B9A44D4).
///
/// Passes the raw call context together with a constant
/// argument-descriptor table pointer to an unpacking trampoline, which
/// unpacks the eleven script arguments and tail-calls the real setter
/// (the trampoline is intercepted and scripted by the checker, so only
/// the two forwarded words are observed here). No return slot is written.
export!(cdecl, rw_00a01e20(ctx: *const u8) -> u32 {
    unsafe {
        // The handler never unpacks the script arguments itself: it hands
        // the raw context plus a constant argument-descriptor table to an
        // unpacking trampoline (intercepted and scripted by the checker).
        // The table address is relocated, never hard-coded.
        callee_cdecl!(1, u32, relocated(0x00A06BE0), ctx as u32)
    }
});
