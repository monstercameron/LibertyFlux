// original: 0x00BB29B0 SET_PLAYER_AS_COP
// SET_PLAYER_AS_COP: forward (player, cop?) with the bool coerced through
// the incoming stack slot.
export!(cdecl, rw_00BB29B0(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a, coerced(ctx, *a.add(1)))
    }
});
