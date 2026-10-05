// original: 0x005688B0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_97, player_schema::LeaderboardInfo, 10>::vf6

/// Virtual method of a `rlConcreteLeaderboardInfo` template instantiation
/// (one ranked-episodic-race leaderboard schema). The original ignores its
/// `this` pointer; it looks up per-schema metadata through an intercepted
/// lookup helper and then derives the result from plain memory reads.
/// Thiscall with one stack argument; the callee pops it (the callee pops 4 bytes).
///
/// Looks up the leaderboard's metadata for field id `LOOKUP_FIELD`, then
/// searches the key array for `value` and returns the found position, or
/// all-ones when the lookup fails, the key count is not positive, or the
/// value is absent.
/// Original: 0x005688B0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_005688b0(_this: u32, value: u32) -> u32 {
    unsafe {
        /// Schema field id this instantiation passes to the lookup helper.
        const LOOKUP_FIELD: u32 = 0x12c;
        /// Intercepted lookup helper (checker callee id 1).
        const LOOKUP_CALLEE: u32 = 1;
        /// Out-buffer word indexes: key count, key array.
        const BUF_COUNT: usize = 0x0C / 4;
        const BUF_KEYS: usize = 0x10 / 4;
        #[inline(always)]
        unsafe fn mem32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut buf = [0u32; 8];
        buf[BUF_COUNT] = 0;
        buf[BUF_KEYS] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(LOOKUP_CALLEE, u32, LOOKUP_FIELD, buf.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return 0xFFFF_FFFF;
        }
        let count = buf[BUF_COUNT];
        if (count as i32) <= 0 {
            return 0xFFFF_FFFF;
        }
        let keys = buf[BUF_KEYS];
        let mut i = 0u32;
        while (i as i32) < (count as i32) {
            if mem32(keys.wrapping_add(i.wrapping_mul(4))) == value {
                return i;
            }
            i = i.wrapping_add(1);
        }
        0xFFFF_FFFF
    }
});
