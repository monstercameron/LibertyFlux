// original: 0x00553a00 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_21, player_schema::LeaderboardInfo, 10>::vf12

/// Find the key of entry IDX in this race's leaderboard key table.
///
/// Asks the loader (kind 0xE0) for this race's tables, reads the element
/// at IDX from the value table (buffer+20) and scans the key table (count
/// at buffer+4, keys at buffer+8) for it, returning the first matching
/// index. Returns -1 when the loader reports failure, the element or the
/// count word reads -1/0 respectively, or the element is absent. Note the
/// count test is equality with zero (a negative count scans on with an
/// unsigned bound), unlike the signed test of the neighbouring finders.
/// The object pointer is unused. Thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_00553a00(_this: u32, idx: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0xE0;
        const OFF_COUNT: u32 = 4;
        const OFF_KEYS: u32 = 8;
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
        let elem = rd32(vals.wrapping_add(idx.wrapping_mul(4)));
        if elem == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = rd32(base + OFF_COUNT);
        if count == 0 {
            return NOT_FOUND;
        }
        let keys = rd32(base + OFF_KEYS);
        let mut i = 0u32;
        while i < count {
            if rd32(keys.wrapping_add(i.wrapping_mul(4))) == elem {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
