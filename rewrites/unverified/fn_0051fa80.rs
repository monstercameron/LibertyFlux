// original: 0x0051fa80 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race33NoHolds, player_schema::LeaderboardInfo, 10>::vf8

/// Leaderboard entry size class: how wide one indexed entry is.
///
/// Classifies the entry at `index` in leaderboard `0x76`'s list through
/// the kind callee: kinds 1 and 5 give 4, kinds 2 and 3 give 8, and
/// anything else (including a failed lookup) gives 0.
///
/// Note: the original dispatches through a jump table the checker cannot
/// serve (unrelocated absolute address), so this rewrite is kept for a
/// later re-run, not verified.
///
/// Original: 0x0051FA80 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0051fa80(index: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const LEADERBOARD_ID: u32 = 0x76;
        const INFO_CALLEE: u32 = 1;
        const KIND_CALLEE: u32 = 2;
        const INFO_LOOKUP: usize = 5;
        let mut info = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok == 0 {
            return 0;
        }
        let lookup = info[INFO_LOOKUP];
        let v = rd32(lookup.wrapping_add(index.wrapping_mul(4)));
        // The original passes its stale second register through to the kind
        // callee; only the first (the entry value) is observed behaviour.
        let kind: u32 = lf_checker_rt::callee_fastcall!(KIND_CALLEE, u32, v, 0);
        if kind == 0xffff_ffff {
            return 0;
        }
        // The original's five-way table: kinds 1 and 5 give 4, kinds 2 and
        // 3 give 8, everything else gives 0 (kind 4 falls to the default).
        match kind.wrapping_sub(1) {
            0 | 4 => 4,
            1 | 2 => 8,
            _ => 0,
        }
    }
});
