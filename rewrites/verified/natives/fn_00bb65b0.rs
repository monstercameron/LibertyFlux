// original: 0x00bb65b0 REGISTER_INT_STAT
/// Script native `REGISTER_INT_STAT` (hash 0x609D07DB).
///
/// Forwards two script arguments (a stat id and a value) to the engine
/// worker. No return slot is written.
export!(cdecl, rw_00bb65b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
