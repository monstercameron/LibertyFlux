// original: 0x00bb6780 UPDATE_NETWORK_RELATIVE_SCORE
/// Script native `UPDATE_NETWORK_RELATIVE_SCORE` (hash 0x384E3F3A).
///
/// Forwards three script arguments to the engine.
/// No return slot is written.
export!(cdecl, rw_00bb6780(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
