// original: 0x005691D0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_99, player_schema::LeaderboardInfo, 10>::vf7

/// Virtual method of a `rlConcreteLeaderboardInfo` template instantiation
/// (one ranked-episodic-race leaderboard schema). The original ignores its
/// `this` pointer; it looks up per-schema metadata through an intercepted
/// lookup helper and then derives the result from plain memory reads.
/// Thiscall with one stack argument; the callee pops it (the callee pops 4 bytes).
///
/// Looks up the leaderboard's row array for field id `LOOKUP_FIELD` and
/// returns row `index`, or all-ones when the lookup reports failure.
/// The lookup helper answers through an out-buffer on the caller's frame
/// (row-array pointer at word `BUF_ROWS`); its address is skipped by the
/// contract and only the compared register argument and the behaviour are
/// checked.
/// Original: 0x005691D0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_005691d0(_this: u32, index: u32) -> u32 {
    unsafe {
        /// Schema field id this instantiation passes to the lookup helper.
        const LOOKUP_FIELD: u32 = 0x12e;
        /// Intercepted lookup helper (checker callee id 1).
        const LOOKUP_CALLEE: u32 = 1;
        /// Word index in the lookup out-buffer holding the row-array pointer.
        const BUF_ROWS: usize = 0x10 / 4;
        #[inline(always)]
        unsafe fn mem32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut buf = [0u32; 8];
        buf[BUF_ROWS] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(LOOKUP_CALLEE, u32, LOOKUP_FIELD, buf.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return 0xFFFF_FFFF;
        }
        let rows = buf[BUF_ROWS];
        mem32(rows.wrapping_add(index.wrapping_mul(4)))
    }
});
