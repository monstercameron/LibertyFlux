// original: 0x00bd43f0 START_PTFX_ON_OBJ
/// Script native `START_PTFX_ON_OBJ` (hash 0x0D8407E9).
///
/// Starts a particle effect on an object. Unusually, this handler does not
/// unpack the argument array itself: it passes a constant dispatch tag and
/// the call-context pointer straight through to the engine worker, which
/// reads the script arguments from the context.
///
/// The tag is a relocated address: the push-immediate instruction carries a
/// relocation entry (Verified: raw relocation-table parse, confirmed by the
/// checker's observation of the relocated value), so the original pushes the
/// mapped address. The rewrite derives it with `relocated()` like any global
/// reference. No return slot is written.
export!(cdecl, rw_00bd43f0(ctx: *const u8) -> u32 {
    const DISPATCH_TAG_FILE_VA: u32 = 0x00BD_6010;
    callee_cdecl!(1, u32, relocated(DISPATCH_TAG_FILE_VA), ctx as u32)
});
