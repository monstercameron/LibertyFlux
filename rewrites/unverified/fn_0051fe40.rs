// original: 0x0051fe40 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race34NoHolds, player_schema::LeaderboardInfo, 10>::vf6

/// Leaderboard index lookup: position of a value in this board's id list.
///
/// Asks the board-info callee for leaderboard `0x77`'s entry list (count
/// then item pointer, filled into a scratch buffer) and returns the index
/// of the first item equal to `want`, or all-ones when the lookup fails,
/// the list is empty or no item matches. The bound is signed: a
/// non-positive count ends the search at once.
///
/// Original: 0x0051FE40 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0051fe40(want: u32) -> u32 {
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
        const INFO_ITEMS: usize = 4;
        const NOT_FOUND: u32 = 0xffff_ffff;
        let mut info = [0u32; 8];
        let ok: u32 = lf_checker_rt::callee_fastcall!(INFO_CALLEE, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let count = info[INFO_COUNT] as i32;
        let items = info[INFO_ITEMS];
        if count <= 0 {
            return NOT_FOUND;
        }
        // Signed bound and increment exactly as the original's loop.
        let mut i: i32 = 0;
        loop {
            let v = rd32(items.wrapping_add((i as u32).wrapping_mul(4)));
            if v == want {
                return i as u32;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});
