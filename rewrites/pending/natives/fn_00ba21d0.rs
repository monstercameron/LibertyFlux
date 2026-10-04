// original: 0x00ba21d0 SET_MONEY_CARRIED_BY_PED_WITH_MODEL
/// Script native `SET_MONEY_CARRIED_BY_PED_WITH_MODEL` (hash 0x047D3BD6).
///
/// Forwards three script arguments to the engine. No return slot is written.
export!(cdecl, rw_00ba21d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
