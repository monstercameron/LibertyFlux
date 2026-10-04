// original: 0x0056cc40 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_113, player_schema::LeaderboardInfo, 10>::vf12
/// Position of `rows[index]` in the id table, or NOT_FOUND.
///
/// Fetches the tables for LEADERBOARD_ID and fails with NOT_FOUND when the
/// fetch fails. Otherwise loads the probe value from the rows table (frame
/// word 5) at `index` and fails when it is the empty marker (-1). Then scans
/// the first `count` entries (frame word 1) of the id table (frame word 2)
/// for the probe and returns the first matching position, or NOT_FOUND. An
/// empty table matches nothing without reading the array; the count is
/// compared unsigned.
///
/// stdcall with one stack argument; no `this`, no globals, no caller-visible
/// writes.
lf_checker_rt::export!(stdcall, rw_0056cc40(index: u32) -> u32 {
    unsafe {
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        const EMPTY_MARKER: u32 = 0xFFFF_FFFF;
        const COUNT_WORD: usize = 1;
        const IDS_WORD: usize = 2;
        const ROWS_WORD: usize = 5;
        const LEADERBOARD_ID: u32 = 316;
        const FETCH_CALLEE: u32 = 1;
        let mut frame = [0u32; 6];
        let answer = lf_checker_rt::callee_fastcall!(FETCH_CALLEE, u32, LEADERBOARD_ID,
            frame.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return NOT_FOUND;
        }
        let rows = frame[ROWS_WORD];
        let probe = (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read();
        if probe == EMPTY_MARKER {
            return NOT_FOUND;
        }
        let count = frame[COUNT_WORD];
        if count == 0 {
            return NOT_FOUND;
        }
        let ids = frame[IDS_WORD];
        let mut pos = 0u32;
        loop {
            let id = (ids.wrapping_add(pos.wrapping_mul(4)) as *const u32).read();
            if id == probe {
                return pos;
            }
            pos += 1;
            if pos >= count {
                return NOT_FOUND;
            }
        }
    }
});
