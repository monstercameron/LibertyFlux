// original: 0x00bb76a0 START_STREAMING_REQUEST_LIST
/// Script native `START_STREAMING_REQUEST_LIST` (hash 0x7858750E).
///
/// Forwards one script argument (a request-list name hash) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bb76a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});

