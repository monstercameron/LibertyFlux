// original: 0x0058F6E0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_239, player_schema::LeaderboardInfo, 10>::vf9
/// Leaderboard element rank: classify the key at position `index`.
/// Calls the leaderboard helper (id 0x1c7) for the key table (record
/// word 5, `+0x14`), reads the key at `index`, and passes it to the
/// classifier helper. The classifier's answer minus one selects 0, 1, 3,
/// -1 or 2; any other answer (including failure of either helper)
/// yields 0xFFFFFFFF.
/// Original: 0x0058F6E0 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0058f6e0(index: u32) -> u32 {
    unsafe {
        const RACE_ID: u32 = 0x1c7;
        const KEYS_WORD: usize = 5;
        const MISS: u32 = 0xFFFF_FFFF;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut info = [0u32; 6];
        let ok = lf_checker_rt::callee_fastcall!(1, u32, RACE_ID, info.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return MISS;
        }
        let elem = rd32(info[KEYS_WORD].wrapping_add(index.wrapping_mul(4)));
        let v = lf_checker_rt::callee_thiscall!(2, u32, elem);
        if v == MISS {
            return MISS;
        }
        match v.wrapping_sub(1) {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => MISS,
            4 => 2,
            _ => MISS,
        }
    }
});
