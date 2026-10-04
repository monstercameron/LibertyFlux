// original: 0x0x00557E20 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_36, player_schema::LeaderboardInfo, 10>::vf6

/// Ranked episodic race 36 leaderboard id search: fetch the board's id
/// list, then return the position of a caller-supplied id inside it.
///
/// The fetch helper fills two buffer words for `BOARD_ID`: `COUNT_SLOT`
/// (offset `0x0c`, a signed id count) and `IDS_SLOT` (offset `0x10`, pointer
/// to the id array); a zero low byte in its answer means failure. `want` is
/// searched for linearly in `ids[0..count]`. The result is the first matching
/// position, or `NOT_FOUND` (`-1`) when the fetch fails, the count is zero
/// or negative, or no id matches.
///
/// The incoming object pointer is ignored. Original: thiscall, one stack
/// word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00557e20(_this: u32, want: u32) -> u32 {
    unsafe {
        const FETCH: u32 = 1;
        const FETCH_WORDS: usize = 6;
        const BOARD_ID: u32 = 0xf0;
        const COUNT_SLOT: usize = 3; // buffer offset 0x0c
        const IDS_SLOT: usize = 4; // buffer offset 0x10
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
                return i;
            }
            i = i.wrapping_add(1);
            if (i as i32) >= count {
                return NOT_FOUND;
            }
        }
    }
});
