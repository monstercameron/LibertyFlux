// original: 0x00577580 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_151, player_schema::LeaderboardInfo, 10>::vf8

/// Classify a leaderboard value slot into a width (4 or 8), or 0.
///
/// `this` (ECX) is ignored; `index` addresses the values table with no
/// bounds check. Fetches the leaderboard tables for id 0x16f
/// through the fetch callee; word 5 of the scratch structure is the
/// values pointer. When the fetch fails (low byte of the answer is zero)
/// returns 0. Otherwise passes `values[index]` to the tag callee; when
/// its answer is -1 returns 0. Otherwise subtracts one and dispatches on
/// the result: 0->0x4, 1->0x8, 2->0x8, 3->0x0, 4->0x4,
/// anything else returns 0.
///
/// Original: 0x00577580 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00577580(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x16f;
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
            return 0;
        }
        let vals = out[5];
        let tag = rd(vals.wrapping_add(index.wrapping_mul(4)));
        let ans: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, tag);
        if ans == MISSING {
            return 0;
        }
        let case = ans.wrapping_sub(1);
        if case > 4 {
            return 0;
        }
        match case {
            0 => 0x4,
            1 => 0x8,
            2 => 0x8,
            3 => 0x0,
            _ => 0x4,
        }
    }
});
