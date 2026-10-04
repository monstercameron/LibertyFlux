// original: 0x0x00557BC0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_36, player_schema::LeaderboardInfo, 10>::vf12

/// Ranked episodic race 36 leaderboard reverse lookup: fetch the board's
/// id list and value table, then return the position of one value inside the
/// id list.
///
/// The fetch helper fills three buffer words for `BOARD_ID`: `COUNT_SLOT`
/// (offset `0x04`, an unsigned id count), `IDS_SLOT` (offset `0x08`, pointer
/// to the id array) and `VALUES_SLOT` (offset `0x14`, pointer to the value
/// array); a zero low byte in its answer means failure. The value at
/// `values[index]` is then searched for linearly in `ids[0..count]`. The
/// result is the first matching position, or `NOT_FOUND` (`-1`) when the
/// fetch fails, the value itself is `-1`, the count is zero, or no id
/// matches.
///
/// The incoming object pointer is ignored. Original: thiscall, one stack
/// word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00557bc0(_this: u32, index: u32) -> u32 {
    unsafe {
        const FETCH: u32 = 1;
        const FETCH_WORDS: usize = 6;
        const BOARD_ID: u32 = 0xf0;
        const COUNT_SLOT: usize = 1; // buffer offset 0x04
        const IDS_SLOT: usize = 2; // buffer offset 0x08
        const VALUES_SLOT: usize = 5; // buffer offset 0x14
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut buf = [0u32; FETCH_WORDS];
        let answer: u32 =
            lf_checker_rt::callee_fastcall!(FETCH, u32, BOARD_ID, buf.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return NOT_FOUND;
        }
        let values = buf[VALUES_SLOT];
        let want = (values.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        if want == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = buf[COUNT_SLOT];
        if count == 0 {
            return NOT_FOUND;
        }
        let ids = buf[IDS_SLOT];
        let mut i: u32 = 0;
        while i < count {
            let id = (ids.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if id == want {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
