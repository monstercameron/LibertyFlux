// original: 0x0056c6f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_111, player_schema::LeaderboardInfo, 10>::vf9

/// Leaderboard entry rank class: a small code for one indexed entry.
///
/// Classifies the entry at `index` in leaderboard `0x13a`'s list through
/// the kind callee: kind 1 gives 0, kind 2 gives 1, kind 3 gives 3, kind 5
/// gives 2, and anything else (kind 4, a failed kind, a failed lookup)
/// gives all-ones.
///
/// Original: 0x0056C6F0 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0056c6f0(index: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const LEADERBOARD_ID: u32 = 0x13a;
        const INFO_CALLEE: u32 = 1;
        const KIND_CALLEE: u32 = 2;
        const INFO_LOOKUP: usize = 5;
        let mut info = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok == 0 {
            return 0xffff_ffff;
        }
        let lookup = info[INFO_LOOKUP];
        let v = rd32(lookup.wrapping_add(index.wrapping_mul(4)));
        // The original passes its stale second register through to the kind
        // callee; only the first (the entry value) is observed behaviour.
        let kind: u32 = lf_checker_rt::callee_fastcall!(KIND_CALLEE, u32, v, 0);
        // The original's five-way table over kinds 1..=5 (a failed kind of
        // all-ones and anything outside the range take the default): kind 1
        // gives 0, kind 2 gives 1, kind 3 gives 3, kind 5 gives 2, and kind
        // 4 falls through to all-ones with the rest of the default arm.
        match kind {
            1 => 0,
            2 => 1,
            3 => 3,
            5 => 2,
            _ => 0xffff_ffff,
        }
    }
});
