// original: 0x0051fc50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race34NoHolds, player_schema::LeaderboardInfo, 10>::vf13

/// Leaderboard mapped lookup: the value paired with a key in this board.
///
/// Finds `want` in leaderboard `0x77`'s key list and returns the value at
/// the same position in its value list, or all-ones when the lookup fails,
/// the list is empty or no key matches. The bound is signed.
///
/// Original: 0x0051FC50 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0051fc50(want: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const LEADERBOARD_ID: u32 = 0x77;
        const INFO_CALLEE: u32 = 1;
        const INFO_COUNT: usize = 3;
        const INFO_KEYS: usize = 4;
        const INFO_VALUES: usize = 5;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut info = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let count = info[INFO_COUNT] as i32;
        let keys = info[INFO_KEYS];
        let values = info[INFO_VALUES];
        if count <= 0 {
            return NOT_FOUND;
        }
        // Signed bound and increment exactly as the original's loop. The
        // original compares the found position against -1 afterwards, which
        // can never match (positions are non-negative); that check is dead.
        let mut i: i32 = 0;
        loop {
            if rd32(keys.wrapping_add((i as u32).wrapping_mul(4))) == want {
                return rd32(values.wrapping_add((i as u32).wrapping_mul(4)));
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});
