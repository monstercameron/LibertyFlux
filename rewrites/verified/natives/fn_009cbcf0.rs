// original: 0x009cbcf0 HANDLE_AUDIO_ANIM_EVENT
/// Script native `HANDLE_AUDIO_ANIM_EVENT` (hash 0x56C15139).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_009cbcf0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
