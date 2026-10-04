// original: 0x00ba21c0 SET_MONEY_CARRIED_BY_ALL_NEW_PEDS
/// Script native `SET_MONEY_CARRIED_BY_ALL_NEW_PEDS` (hash 0x64CA2868).
///
/// Forwards one script argument (an amount of money) to the engine. No
/// return slot is written.
export!(cdecl, rw_00ba21c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
