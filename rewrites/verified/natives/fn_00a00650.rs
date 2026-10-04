// original: 0x00a00650 CONNECT_LODS
/// Script native `CONNECT_LODS` (hash 0x79EB2BC9).
///
/// Forwards two script arguments (two object handles whose LODs are linked)
/// to the engine. No return slot is written.
export!(cdecl, rw_00a00650(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
