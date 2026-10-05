// original: 0x00554120 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_22, player_schema::LeaderboardInfo, 10>::vf7

/// Fetch entry IDX from this race's leaderboard value table.
///
/// Asks the loader (kind 0xE1) for this race's value table (table at
/// buffer+16) and returns the word at IDX with no bounds check. Returns -1
/// only when the loader reports failure. The object pointer is unused.
/// Thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_00554120(_this: u32, idx: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0xE1;
        const OFF_VALS: u32 = 16;
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
        rd32(vals.wrapping_add(idx.wrapping_mul(4)))
    }
});
