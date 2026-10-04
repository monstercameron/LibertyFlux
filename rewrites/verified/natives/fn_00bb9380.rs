// original: 0x00bb9380 TASK_CHAR_SLIDE_TO_COORD_AND_PLAY_ANIM
/// Script native `TASK_CHAR_SLIDE_TO_COORD_AND_PLAY_ANIM` (hash 0x79BB1D64).
///
/// Unlike the unpack-and-call handlers, this handler passes its own context
/// pointer together with the engine worker's address to a shared adapter
/// thunk, which unpacks the script arguments itself. The context pointer is
/// pushed first, so in cdecl argument order the worker address is the first
/// argument and the context pointer the second. The worker address is
/// pushed as an immediate that carries a base relocation (Verified: the
/// relocation table has a HIGHLOW entry at the immediate), so it is derived
/// with `relocated()` rather than hard-coded. No return slot is written.
export!(cdecl, rw_00bb9380(ctx: *const u8) -> u32 {
    callee_cdecl!(1, u32, relocated(0x00BBD1F0), ctx as u32)
});
