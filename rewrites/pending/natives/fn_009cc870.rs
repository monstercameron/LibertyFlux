// original: 0x009cc870 STOP_END_CREDITS_MUSIC
/// Script native `STOP_END_CREDITS_MUSIC` (hash 0x47E93CB8).
///
/// Takes no script arguments: the handler body is a single jump to a shared
/// engine routine. The rewrite expresses that tail jump as a call that
/// forwards no arguments and returns the shared routine's answer.
export!(cdecl, rw_009cc870(ctx: *const u8) -> u32 {
    let _ = ctx;
    callee_cdecl!(1, u32,)
});
