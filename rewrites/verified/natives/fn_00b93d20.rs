// original: 0x00b93d20 ACTIVATE_SAVE_MENU
/// Script native `ACTIVATE_SAVE_MENU` (hash 0x78AC735F).
///
/// Takes no script arguments: tail-jumps to the shared save-menu engine routine, returning its answer.
export!(cdecl, rw_00b93d20(ctx: *const u8) -> u32 {
    // Tail jump: the context pointer is passed through untouched;
    // the engine routine it reaches takes no stack arguments.
    let _ = ctx;
    callee_cdecl!(1, u32,)
});
