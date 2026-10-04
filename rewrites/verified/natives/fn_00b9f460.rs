// original: 0x00b9f460 GET_PED_BONE_POSITION
/// Script native `GET_PED_BONE_POSITION` (hash 0x43475BB3).
///
/// Unlike most handlers this one does not unpack the argument
/// array: it forwards an engine address plus the call-context
/// pointer itself to a shared engine worker, in that order (the
/// context is pushed first, so it is the second call argument).
/// The pushed constant is a relocated code address, resolved
/// through the checker's image base, never hard-coded.
/// No return slot is written.
export!(cdecl, rw_00b9f460(ctx: *const u8) -> u32 {
    unsafe {
        callee_cdecl!(1, u32, relocated(0x00BA75B0), ctx as u32)
    }
});
