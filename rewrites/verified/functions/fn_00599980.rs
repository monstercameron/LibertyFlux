// original: 0x00599980 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Standard_Episodic_7, player_schema::LeaderboardInfo, 10>::vf2
/// Resolve this leaderboard instance's stored resource address when its
/// virtual query method returns the requested key. The object is passed in
/// ECX; the output pointer and expected 32-bit key are stack arguments. A
/// null output pointer or an unequal key returns zero. On a match, the fixed
/// image datum address is written through the output pointer and that pointer
/// is returned. The call through vtable slot one is represented by the
/// checker's scripted thiscall callee.
lf_checker_rt::export!(thiscall, rw_00599980(this: u32, output: u32, expected_key: u32) -> u32 {
    const QUERY_CALLEE: u32 = 1;
    const RESOURCE_ADDRESS_VA: u32 = 0x00FDB79C;

    let actual_key = lf_checker_rt::callee_thiscall!(QUERY_CALLEE, u32, this);
    if actual_key != expected_key || output == 0 {
        return 0;
    }

    let resource_address = lf_checker_rt::relocated(RESOURCE_ADDRESS_VA);
    unsafe { (output as *mut u32).write_unaligned(resource_address); }
    output
});
