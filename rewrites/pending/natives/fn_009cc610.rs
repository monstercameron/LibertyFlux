// original: 0x009cc610 SET_PLAYER_PAIN_ROOT_BANK_NAME
/// Script native `SET_PLAYER_PAIN_ROOT_BANK_NAME` (hash 0x70AF1D38).
///
/// Forwards one bank-name string hash to the engine. No return slot is
/// written.
export!(cdecl, rw_009cc610(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
        )
    }
});
