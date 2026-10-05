// original: 0x005531c0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_19, player_schema::LeaderboardInfo, 10>::vf13

/// Map KEY to its leaderboard value through this race's key/value tables.
///
/// Asks the loader (kind 0xDE) for this race's tables, then scans the key
/// table (count at buffer+12, keys at buffer+16) for KEY and returns the
/// value at the same index in the value table (buffer+20). Returns -1 when
/// the loader reports failure, the count is zero or negative, or KEY is
/// absent. The incoming object pointer is unused. Thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_005531c0(_this: u32, key: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0xDE;
        const OFF_COUNT: u32 = 12;
        const OFF_KEYS: u32 = 16;
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
        let count = rd32(base + OFF_COUNT) as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = rd32(base + OFF_KEYS);
        let mut i = 0u32;
        while (i as i32) < count {
            if rd32(keys.wrapping_add(i.wrapping_mul(4))) == key {
                let vals = rd32(base + OFF_VALS);
                return rd32(vals.wrapping_add(i.wrapping_mul(4)));
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
