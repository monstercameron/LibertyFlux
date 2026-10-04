// original: 0x00bb6450 INCREMENT_INT_STAT
/// Script native `INCREMENT_INT_STAT`.
///
/// Forwards two script arguments (a statistic id and the increment) to the
/// engine. No return slot is written.
export!(cdecl, rw_00bb6450(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
