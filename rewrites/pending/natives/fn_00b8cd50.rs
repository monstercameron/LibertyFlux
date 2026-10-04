// original: 0x00b8cd50 IS_REPLAY_SAVING
/// Reports whether a replay is currently saving. Takes no script arguments; stores the engine's boolean answer in the return slot.
///
/// Native handler: the script VM passes one pointer to a call context.
/// The context holds the return-slot pointer at offset 0 and the argument
/// array pointer at offset 8. Integer arguments pass through as raw words;
/// float arguments pass through bitwise.
checker_rt::export!(cdecl, rw_00b8cd50(ctx: *const u8) -> u32 {
    unsafe {
        let argv = *((ctx.add(8)) as *const *const u32);
        let answer: u32 = checker_rt::callee_cdecl!(1, u32,);
        // Only the low byte of the engine answer is defined; the rest is discarded.
        let ret_slot = *(ctx as *const *mut u32);
        *ret_slot = answer & 0xFF;
        ret_slot as u32
    }
});
