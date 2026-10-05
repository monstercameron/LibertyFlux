// original: 0x00bb6700 SET_STAT_FRONTEND_NEVER_VISIBLE
/// Script native `SET_STAT_FRONTEND_NEVER_VISIBLE` (hash 0x3A6B0308).
///
/// Forwards one script argument (a statistic handle) to the engine. No
/// return slot is written. (The original cleans its one pushed argument
/// with `(an instruction of the original)`; the effect on the stack pointer is identical to the
/// plain cdecl return here.)
export!(cdecl, rw_00bb6700(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
