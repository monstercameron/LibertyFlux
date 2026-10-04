// original: 0x00555BA0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_28, player_schema::LeaderboardInfo, 10>::vf8

/// Classify one leaderboard entry into a size class (0, 4 or 8).
///
/// `index` selects a row of the entry table. The info callee (id 1, called with
/// `LEADERBOARD_ID` in ecx and a pointer to a six-word out-struct in edx) fills
/// `TABLE` (struct word 5); the word is zeroed before the call. The selected
/// entry goes to the key callee (id 2, thiscall of one word); its answer minus
/// one indexes a five-way dispatch: answers 1 and 5 give 4, answers 2 and 3
/// give 8, answer 4 gives 0. Any other answer (including -1, 0 and anything
/// above 5), and a failed info call (al == 0), give 0.
///
/// Original: 0x00555BA0 (stdcall, one stack word; ecx is ignored).
lf_checker_rt::export!(stdcall, rw_00555BA0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xef;
        const INFO_CALLEE: u32 = 1;
        const KEY_CALLEE: u32 = 2;
        let mut info = [0u32; 6];
        info[5] = 0; // TABLE
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32,
            LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return 0;
        }
        let raw = (info[5].wrapping_add(index.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let key: u32 = lf_checker_rt::callee_thiscall!(KEY_CALLEE, u32, raw);
        if key == 0xFFFF_FFFF {
            return 0;
        }
        match key.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            4 => 4,
            _ => 0,
        }
    }
});
