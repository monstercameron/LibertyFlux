// original: 0x0051A8C0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race15NoHolds, player_schema::LeaderboardInfo, 10>::vf2

/// Publish this leaderboard's tag into a caller slot after checking the
/// schema probe against the caller's key.
///
/// `this` is the leaderboard object; the probe is its virtual slot at
/// `+4`, called with the object. When the probe's answer differs from
/// `key`, or `slot` is null, nothing is stored and the result is 0.
/// Otherwise the address of this leaderboard's tag record (file
/// address `0x00fd1ca4`, relocated at load) is stored through `slot`
/// and `slot` itself is returned.
///
/// Original: 0x0051A8C0 (thiscall: object in `ecx`, two stack words).
lf_checker_rt::export!(thiscall, rw_0051a8c0(this: u32, slot: u32, key: u32) -> u32 {
    unsafe {
        // File address of this leaderboard's tag record; relocated at load.
        const TAG: u32 = 0x00fd1ca4;

        let vtable = (this as *const u32).read_unaligned();
        let target = ((vtable.wrapping_add(4)) as *const u32).read_unaligned();
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        if probe(this) != key {
            return 0;
        }
        if slot == 0 {
            return 0;
        }
        (slot as *mut u32).write_unaligned(lf_checker_rt::relocated(TAG));
        slot
    }
});
