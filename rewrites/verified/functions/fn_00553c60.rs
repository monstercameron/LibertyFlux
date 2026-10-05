// original: 0x00553c60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_21, player_schema::LeaderboardInfo, 10>::vf6

/// Find KEY in this race's leaderboard key table, returning its index.
///
/// Asks the loader (kind 0xE0) for this race's key table (count at
/// buffer+12, keys at buffer+16) and scans it for KEY, returning the first
/// matching index. Returns -1 when the loader reports failure, the count
/// is zero or negative, or KEY is absent. The object pointer is unused.
/// Thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_00553c60(_this: u32, key: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0xE0;
        const OFF_COUNT: u32 = 12;
        const OFF_KEYS: u32 = 16;
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
        let count = rd32(base + OFF_COUNT) as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = rd32(base + OFF_KEYS);
        let mut i = 0u32;
        while (i as i32) < count {
            if rd32(keys.wrapping_add(i.wrapping_mul(4))) == key {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
