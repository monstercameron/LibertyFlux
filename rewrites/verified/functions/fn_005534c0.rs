// original: 0x005534c0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_19, player_schema::LeaderboardInfo, 10>::vf9

/// Classify entry IDX of this race's leaderboard through the kind probe.
///
/// Asks the loader (kind 0xDE) for this race's value table (table at
/// buffer+20), passes the word at IDX to the kind probe and maps its 1..=5
/// answer to a class (0, 1, 3, -1, 2). Returns -1 when the loader reports
/// failure, the probe reports -1, or its answer falls outside 1..=5. The
/// object pointer is unused. Thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_005534c0(_this: u32, idx: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0xDE;
        const OFF_VALS: u32 = 20;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut out = [0u32; 8];
        let base = out.as_mut_ptr() as u32;
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, KIND, base);
        if (ok & 0xFF) == 0 {
            return NOT_FOUND;
        }
        let vals = rd32(base + OFF_VALS);
        let probe: u32 = lf_checker_rt::callee_thiscall!(2, u32,
            rd32(vals.wrapping_add(idx.wrapping_mul(4))));
        if probe == NOT_FOUND {
            return NOT_FOUND;
        }
        match probe.wrapping_sub(1) {
                0 => 0,
                1 => 1,
                2 => 3,
                3 => 0xFFFF_FFFF,
                4 => 2,
            _ => NOT_FOUND,
        }
    }
});
