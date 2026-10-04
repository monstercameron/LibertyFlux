// original: 0x00b8bbc0 ADD_STRING_WITH_THIS_TEXT_LABEL_TO_PREVIOUS_BRIEF
/// Script native `ADD_STRING_WITH_THIS_TEXT_LABEL_TO_PREVIOUS_BRIEF` (hash 0x76860554).
///
/// Forwards one script argument (a text-label string) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b8bbc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
