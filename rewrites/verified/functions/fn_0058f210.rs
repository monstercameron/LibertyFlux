// original: 0x0058F210 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_238, player_schema::LeaderboardInfo, 10>::vf8
/// Leaderboard element width: classify the key at position `index`.
/// Calls the leaderboard helper (id 0x1c6) for the key table (record
/// word 5, `+0x14`), reads the key at `index`, and passes it to the
/// classifier helper. The classifier's answer minus one selects 4, 8, 8, 0
/// or 4; any other answer (including failure of either helper) yields 0.
/// Original: 0x0058F210 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0058f210(index: u32) -> u32 {
    unsafe {
        const RACE_ID: u32 = 0x1c6;
        const KEYS_WORD: usize = 5;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut info = [0u32; 6];
        let ok = lf_checker_rt::callee_fastcall!(1, u32, RACE_ID, info.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return 0;
        }
        let elem = rd32(info[KEYS_WORD].wrapping_add(index.wrapping_mul(4)));
        let v = lf_checker_rt::callee_thiscall!(2, u32, elem);
        if v == 0xFFFF_FFFF {
            return 0;
        }
        match v.wrapping_sub(1) {
            0 => 4,
            1 => 8,
            2 => 8,
            3 => 0,
            4 => 4,
            _ => 0,
        }
    }
});
