// original: 0x00576d30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_149, player_schema::LeaderboardInfo, 10>::vf9

/// Classify a leaderboard value slot into a small code, or -1.
///
/// `this` (ECX) is ignored; `index` addresses the values table with no
/// bounds check. Fetches the leaderboard tables for id 0x16d
/// through the fetch callee; word 5 of the scratch structure is the
/// values pointer. When the fetch fails (low byte of the answer is zero)
/// returns -1. Otherwise passes `values[index]` to the tag callee; when
/// its answer is -1 returns -1. Otherwise subtracts one and dispatches on
/// the result: 0->0x0, 1->0x1, 2->0x3, 3->MISSING, 4->0x2,
/// anything else returns -1.
///
/// Original: 0x00576d30 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00576d30(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x16d;
        const FETCH: u32 = 0;
        const CLASSIFY: u32 = 1;
        const MISSING: u32 = 0xFFFF_FFFF;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = [0u32; 6];
        out[5] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32
        );
        if ok as u8 == 0 {
            return MISSING;
        }
        let vals = out[5];
        let tag = rd(vals.wrapping_add(index.wrapping_mul(4)));
        let ans: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, tag);
        if ans == MISSING {
            return MISSING;
        }
        let case = ans.wrapping_sub(1);
        if case > 4 {
            return MISSING;
        }
        match case {
            0 => 0x0,
            1 => 0x1,
            2 => 0x3,
            3 => MISSING,
            _ => 0x2,
        }
    }
});
