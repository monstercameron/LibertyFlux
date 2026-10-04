// original: 0x00bbacd0 TASK_TURN_CHAR_TO_FACE_CHAR
/// TASK_TURN_CHAR_TO_FACE_CHAR: face-task between two peds.
///
/// Native handler. Forwards both ped handles to the task engine.
export!(cdecl, rw_00bbacd0(ctx: u32) -> u32 {
    unsafe {
        // Native call context: +0 = return-slot pointer, +8 = arg array.
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        callee_cdecl!(1, u32, a0, a1)
    }
});
