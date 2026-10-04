// original: 0x00bc5150 ATTACH_CAR_TO_CAR_PHYSICALLY
/// Script native `ATTACH_CAR_TO_CAR_PHYSICALLY` (hash 0x778F46E3).
///
/// Passes the engine worker address and the whole call context to the shared
/// native-argument unpacker, in that order. Unlike the other handlers it
/// reads no script arguments itself: the unpacker does that. No return
/// slot is written.
export!(cdecl, rw_00bc5150(ctx: *const u8) -> u32 {
    unsafe {
        // The unpacker receives the engine worker address first and
        // the context pointer second (push order is ctx, then the
        // worker address, so the worker address lands on top of the
        // stack). The pushed worker address is relocated with the
        // image, hence `relocated` rather than a literal.
        callee_cdecl!(1, u32, relocated(0x00bc91f0), ctx as u32)
    }
});
