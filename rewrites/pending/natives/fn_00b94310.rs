// original: 0x00b94310 GENERATE_RANDOM_INT
/// Script native `GENERATE_RANDOM_INT` (hash 0x335D0F34).
///
/// Forwards one script argument (the upper bound) to the engine random
/// source. No return slot is written.
export!(cdecl, rw_00b94310(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
