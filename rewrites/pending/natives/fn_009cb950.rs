// original: 0x009cb950 ADD_LINE_TO_SCRIPTED_CONVERSATION
/// Script native `ADD_LINE_TO_SCRIPTED_CONVERSATION` (hash 0x416413F6).
///
/// Forwards three script arguments (conversation slot, line text hash and
/// flags) to the engine. No return slot is written.
export!(cdecl, rw_009cb950(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
        )
    }
});
