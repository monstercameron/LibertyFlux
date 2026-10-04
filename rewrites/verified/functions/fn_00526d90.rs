// original: 0x00526d90 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race60NoHolds, player_schema::LeaderboardInfo, 10>::vf12

/// Find the leaderboard row id selected by an index.
///
/// `index` selects one entry of a row-id array; the query callee fills a
/// six-word scratch struct (word 1 = count, word 2 = id array, word 5 =
/// row array, the rest unused) for leaderboard id `QUERY_ID`. Returns the
/// position of the selected row id inside the id array, or -1 when the
/// query fails, the selected id is -1, the array is empty, or the id is
/// absent.
///
/// Original: stdcall with one stack word; entry registers are ignored.
/// The comparison is unsigned (`jb`); the query address is skipped (it
/// points into each side's own scratch) and its pre-call zeroed words are
/// snapshotted instead.
lf_checker_rt::export!(stdcall, rw_00526d90(index: u32) -> u32 {
    unsafe {
        const QUERY_ID: u32 = 0x4c;
        const CALLEE_QUERY: u32 = 1;
        const NOT_FOUND: u32 = 0xffff_ffff;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut st = [0u32; 6];
        let r: u32 = lf_checker_rt::callee_fastcall!(CALLEE_QUERY, u32, QUERY_ID, st.as_mut_ptr() as u32);
        if (r & 0xff) == 0 {
            return NOT_FOUND;
        }
        let count = st[1];
        let ids = st[2];
        let rows = st[5];
        let want = rd32(rows.wrapping_add(index.wrapping_mul(4)));
        if want == NOT_FOUND {
            return NOT_FOUND;
        }
        if count == 0 {
            return NOT_FOUND;
        }
        let mut i = 0u32;
        loop {
            if rd32(ids.wrapping_add(i.wrapping_mul(4))) == want {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});
