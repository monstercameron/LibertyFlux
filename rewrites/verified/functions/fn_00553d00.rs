// original: 0x00553d00 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_21, player_schema::LeaderboardInfo, 10>::vf8

/// Classify entry IDX of this race's leaderboard through the kind probe.
///
/// Asks the loader (kind 0xE0) for this race's value table (table at
/// buffer+20), passes the word at IDX to the kind probe and maps its 1..=5
/// answer to a class (4, 8, 8, 0, 4). Returns 0 when the loader reports
/// failure, the probe reports -1, or its answer falls outside 1..=5. The
/// object pointer is unused. Thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_00553d00(_this: u32, idx: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0xE0;
        const OFF_VALS: u32 = 20;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut out = [0u32; 8];
        let base = out.as_mut_ptr() as u32;
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, KIND, base);
        if (ok & 0xFF) == 0 {
            return 0;
        }
        let vals = rd32(base + OFF_VALS);
        let probe: u32 = lf_checker_rt::callee_thiscall!(2, u32,
            rd32(vals.wrapping_add(idx.wrapping_mul(4))));
        if probe == 0xFFFF_FFFF {
            return 0;
        }
        match probe.wrapping_sub(1) {
                0 => 4,
                1 => 8,
                2 => 8,
                3 => 0,
                4 => 4,
            _ => 0,
        }
    }
});
