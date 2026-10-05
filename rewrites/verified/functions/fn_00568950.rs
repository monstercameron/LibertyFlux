// original: 0x00568950 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_97, player_schema::LeaderboardInfo, 10>::vf8

/// Virtual method of a `rlConcreteLeaderboardInfo` template instantiation
/// (one ranked-episodic-race leaderboard schema). The original ignores its
/// `this` pointer; it looks up per-schema metadata through an intercepted
/// lookup helper and then derives the result from plain memory reads.
/// Thiscall with one stack argument; the callee pops it (the callee pops 4 bytes).
///
/// Looks up the leaderboard's row array for field id `LOOKUP_FIELD`, reads
/// row `index` and classifies it through an intercepted kind helper; the
/// helper's answer maps to a size (1->4, 2->8, 3->8, 5->4 bytes) and anything
/// else, including lookup failure, yields 0.
/// Original: 0x00568950 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00568950(_this: u32, index: u32) -> u32 {
    unsafe {
        /// Schema field id this instantiation passes to the lookup helper.
        const LOOKUP_FIELD: u32 = 0x12c;
        /// Intercepted lookup helper (checker callee id 1).
        const LOOKUP_CALLEE: u32 = 1;
        /// Intercepted row-kind helper (checker callee id 2).
        const KIND_CALLEE: u32 = 2;
        /// Word index in the lookup out-buffer holding the row-array pointer.
        const BUF_ROWS: usize = 0x14 / 4;
        #[inline(always)]
        unsafe fn mem32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut buf = [0u32; 8];
        buf[BUF_ROWS] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(LOOKUP_CALLEE, u32, LOOKUP_FIELD, buf.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return 0;
        }
        let rows = buf[BUF_ROWS];
        let v = mem32(rows.wrapping_add(index.wrapping_mul(4)));
        let kind: u32 = lf_checker_rt::callee_thiscall!(KIND_CALLEE, u32, v);
        if kind == 0xFFFF_FFFF {
            return 0;
        }
        match kind.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            4 => 4,
            _ => 0,
        }
    }
});
