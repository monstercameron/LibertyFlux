// original: 0x00bd97d0 TELL_NET_PLAYER_TO_START_PLAYING
/// Script native `TELL_NET_PLAYER_TO_START_PLAYING` (hash 0x465D424D).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bd97d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
