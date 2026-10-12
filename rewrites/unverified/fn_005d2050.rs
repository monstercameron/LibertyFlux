// original: 0x005D2050 player_is_wanted

/// Report whether the current player is wanted. The player-ped helper returns
/// a pointer whose signed word at offset 0x6C is tested first; if that value
/// is not positive, a second helper supplies a signed status that is compared
/// against 1. The signed global status is the final fallback. The result is
/// returned in AL by this zero-argument cdecl helper.
lf_checker_rt::export!(cdecl, rw_005d2050() -> u32 {
    const GET_PLAYER_PED: u32 = 1;
    const QUERY_PLAYER_WANTED: u32 = 2;
    const PLAYER_WANTED_LEVEL_OFFSET: u32 = 0x6C;
    const GLOBAL_WANTED_STATUS_VA: u32 = 0x011DB240;

    let first_player = lf_checker_rt::callee_cdecl!(GET_PLAYER_PED, u32,);
    let player_level = unsafe {
        (first_player.wrapping_add(PLAYER_WANTED_LEVEL_OFFSET) as *const u32).read_unaligned()
    };
    if (player_level as i32) > 0 {
        return 1;
    }

    let second_player = lf_checker_rt::callee_cdecl!(GET_PLAYER_PED, u32,);
    let queried_status = lf_checker_rt::callee_thiscall!(QUERY_PLAYER_WANTED, u32, second_player);
    let global_status = unsafe {
        lf_checker_rt::global::<i32>(GLOBAL_WANTED_STATUS_VA).read_unaligned()
    };
    u32::from((queried_status as i32) >= 1 || global_status >= 1)
});
