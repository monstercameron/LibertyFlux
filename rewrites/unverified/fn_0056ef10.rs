// original: 0x0056ef10 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_121, player_schema::LeaderboardInfo, 10>::vf2

/// Publish this leaderboard's record address when the caller names it.
///
/// `this` is the leaderboard info object, `out` an optional result slot
/// and `expected` a leaderboard id. Reads the object's type id through
/// its virtual slot 1; when it differs from `expected`, or when `out` is
/// null, the result is 0 and nothing is stored. Otherwise the address of
/// this leaderboard's record (file address 0xfce82c, relocated at load)
/// is stored to `out` and `out` is returned.
///
/// Original: 0x0056ef10 (rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_121, player_schema::LeaderboardInfo, 10>::vf2; thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0056ef10(this: u32, out: u32, expected: u32) -> u32 {
    unsafe {
            const LEADERBOARD_RECORD: u32 = 0xfce82c;
            const VSLOT_BYTES: u32 = 4;
            let vtable = (this as *const u32).read();
            let slot = ((vtable as u32).wrapping_add(VSLOT_BYTES) as *const u32).read();
            let get_id: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot as usize);
            let id = get_id(this);
            if id != expected {
                return 0;
            }
            if out == 0 {
                return 0;
            }
            (out as *mut u32).write(lf_checker_rt::relocated(LEADERBOARD_RECORD));
            out
    }
});
