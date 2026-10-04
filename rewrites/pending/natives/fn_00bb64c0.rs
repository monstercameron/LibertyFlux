// original: 0x00bb64c0 PLAYSTATS_INT
/// Script native `PLAYSTATS_INT` (hash 0x41FA2D0C).
///
/// Forwards two script arguments to the stat-packet builder and sender.
/// No return slot is written.
export!(cdecl, rw_00bb64c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
