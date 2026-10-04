// original: 0x009cbcc0 GET_STREAM_BEAT_INFO
/// Script native `GET_STREAM_BEAT_INFO` (hash 0x6A3A2C88).
///
/// Forwards three script arguments to the engine. No return slot is written.
export!(cdecl, rw_009cbcc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
