// original: 0x00567D90 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_95, player_schema::LeaderboardInfo, 10>::vf12

/// Virtual method of a `rlConcreteLeaderboardInfo` template instantiation
/// (one ranked-episodic-race leaderboard schema). The original ignores its
/// `this` pointer; it looks up per-schema metadata through an intercepted
/// lookup helper and then derives the result from plain memory reads.
/// Thiscall with one stack argument; the callee pops it (the callee pops 4 bytes).
///
/// Looks up the leaderboard's metadata for field id `LOOKUP_FIELD`: a row
/// count is not used here; instead the row array yields the key for row
/// `index`, and the key array is searched linearly for that key. Returns the
/// found position, or all-ones when the lookup fails, the key is all-ones,
/// the key count is zero, or the key is absent.
/// Original: 0x00567D90 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00567d90(_this: u32, index: u32) -> u32 {
    unsafe {
        /// Schema field id this instantiation passes to the lookup helper.
        const LOOKUP_FIELD: u32 = 0x12a;
        /// Intercepted lookup helper (checker callee id 1).
        const LOOKUP_CALLEE: u32 = 1;
        /// Out-buffer word indexes: key count, key array, row array.
        const BUF_COUNT: usize = 0x04 / 4;
        const BUF_KEYS: usize = 0x08 / 4;
        const BUF_ROWS: usize = 0x14 / 4;
        #[inline(always)]
        unsafe fn mem32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut buf = [0u32; 8];
        buf[BUF_COUNT] = 0;
        buf[BUF_KEYS] = 0;
        buf[BUF_ROWS] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(LOOKUP_CALLEE, u32, LOOKUP_FIELD, buf.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return 0xFFFF_FFFF;
        }
        let rows = buf[BUF_ROWS];
        let key = mem32(rows.wrapping_add(index.wrapping_mul(4)));
        if key == 0xFFFF_FFFF {
            return 0xFFFF_FFFF;
        }
        let count = buf[BUF_COUNT];
        if count == 0 {
            return 0xFFFF_FFFF;
        }
        let keys = buf[BUF_KEYS];
        let mut i = 0u32;
        while i < count {
            if mem32(keys.wrapping_add(i.wrapping_mul(4))) == key {
                return i;
            }
            i = i.wrapping_add(1);
        }
        0xFFFF_FFFF
    }
});
