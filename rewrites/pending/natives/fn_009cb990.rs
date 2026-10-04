// original: 0x009cb990 ADD_NEW_FRONTEND_CONVERSATION_SPEAKER
/// Script native `ADD_NEW_FRONTEND_CONVERSATION_SPEAKER` (hash 0x13D44996).
///
/// Forwards two script arguments (a speaker handle and a conversation index)
/// to the engine. No return slot is written.
export!(cdecl, rw_009cb990(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
