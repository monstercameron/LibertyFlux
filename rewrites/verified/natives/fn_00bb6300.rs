// original: 0x00bb6300 DECREMENT_INT_STAT
/// Script native `DECREMENT_INT_STAT` (hash 0x7DD91295).
///
/// Forwards two script arguments (a stat id and a decrement) to the engine. No return slot is written.
export!(cdecl, rw_00bb6300(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
