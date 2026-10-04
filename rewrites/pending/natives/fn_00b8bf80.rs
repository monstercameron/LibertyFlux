// original: 0x00b8bf80 DEACTIVATE_FRONTEND
/// Script native `DEACTIVATE_FRONTEND` (hash 0x72B16D0D).
///
/// Takes no script arguments: tail-jumps to the shared frontend-deactivate engine routine, returning its answer.
export!(cdecl, rw_00b8bf80(ctx: *const u8) -> u32 {
    // Tail jump: the context pointer is passed through untouched;
    // the engine routine it reaches takes no stack arguments.
    let _ = ctx;
    callee_cdecl!(1, u32,)
});
