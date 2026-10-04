// original: 0x009cc1c0 PLAY_STREAM_FROM_OBJECT
/// Script native `PLAY_STREAM_FROM_OBJECT` (hash 0x4AA86394).
///
/// Forwards one object handle to the engine. No return slot is written.
export!(cdecl, rw_009cc1c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
        )
    }
});
