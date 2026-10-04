// original: 0x00bb7640 SET_STREAMING_REQUEST_LIST_TIME
/// Script native `SET_STREAMING_REQUEST_LIST_TIME` (hash 0x01FF6618).
///
/// Forwards one script argument (a time value) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bb7640(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
