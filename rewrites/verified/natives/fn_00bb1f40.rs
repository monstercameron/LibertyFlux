// original: 0x00bb1f40 GET_PLAYER_ID_FOR_THIS_PED
// rw_get_player_id_for_this_ped: native GET_PLAYER_ID_FOR_THIS_PED (handler 0x00BB1F40).
//
// Forwards a ped handle to the player-index lookup, stores the result in the return slot.
export!(cdecl, rw_get_player_id_for_this_ped(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *mut *const u32);
        let ans = callee_cdecl!(1, u32, *args.add(0));
        *(*ctx as *mut u32) = ans;
        ans
    }
});
