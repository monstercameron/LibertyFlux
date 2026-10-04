// original: 0x0x005577D0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_35, player_schema::LeaderboardInfo, 10>::vf13

/// Ranked episodic race 35 leaderboard value lookup: fetch the board's
/// id list and value table, find a caller-supplied id, and return the value
/// stored at the same position.
///
/// The fetch helper fills three buffer words for `BOARD_ID`: `COUNT_SLOT`
/// (offset `0x0c`, a signed id count), `IDS_SLOT` (offset `0x10`, pointer to
/// the id array) and `VALUES_SLOT` (offset `0x14`, pointer to the value
/// array); a zero low byte in its answer means failure. `want` is searched
/// for linearly in `ids[0..count]` and the result is `values[position]`, or
/// `NOT_FOUND` (`-1`) when the fetch fails, the count is zero or negative,
/// or no id matches. (The original re-checks the found position against -1,
/// which a forward search can never produce; the check is kept for shape.)
///
/// The incoming object pointer is ignored. Original: thiscall, one stack
/// word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_005577d0(_this: u32, want: u32) -> u32 {
    unsafe {
        const FETCH: u32 = 1;
        const FETCH_WORDS: usize = 6;
        const BOARD_ID: u32 = 0xf1;
        const COUNT_SLOT: usize = 3; // buffer offset 0x0c
        const IDS_SLOT: usize = 4; // buffer offset 0x10
        const VALUES_SLOT: usize = 5; // buffer offset 0x14
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut buf = [0u32; FETCH_WORDS];
        let answer: u32 =
            lf_checker_rt::callee_fastcall!(FETCH, u32, BOARD_ID, buf.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return NOT_FOUND;
        }
        let count = buf[COUNT_SLOT] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let ids = buf[IDS_SLOT];
        let mut i: u32 = 0;
        loop {
            let id = (ids.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if id == want {
                if i == NOT_FOUND {
                    return NOT_FOUND;
                }
                let values = buf[VALUES_SLOT];
                return (values.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            }
            i = i.wrapping_add(1);
            if (i as i32) >= count {
                return NOT_FOUND;
            }
        }
    }
});
