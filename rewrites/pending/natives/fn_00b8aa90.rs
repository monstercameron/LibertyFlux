// original: 0x00b8aa90 CLEAR_CUTSCENE
/// Script native `CLEAR_CUTSCENE` (hash 0x79611458).
///
/// Takes no script arguments: tail-jumps to the shared cutscene-clear engine routine, returning its answer.
export!(cdecl, rw_00b8aa90(ctx: *const u8) -> u32 {
    // Tail jump: the context pointer is passed through untouched;
    // the engine routine it reaches takes no stack arguments.
    let _ = ctx;
    callee_cdecl!(1, u32,)
});
