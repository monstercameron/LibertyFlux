// original: 0x005479C0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_4, player_schema::LeaderboardInfo, 10>::vf12
/// Row-position lookup for one ranked leaderboard board (virtual slot 12).
///
/// Fetches the board tables for board id 0xB5, takes the row word at
/// `index` (`rows[index]`) as a key, and returns the position of the first
/// equal word in the id array, or -1 when absent.
///
/// Frame layout (offsets from the out-struct pointer): the entry count lands
/// at `+0x04`, the id-array pointer at `+0x08`, the rows-table pointer at
/// `+0x14`. The fetch answer's low byte decides failure (-1). A key of -1
/// yields -1 without searching, as does a zero count; the count is compared
/// unsigned.
/// The shared fetch step fills a six-word frame passed by pointer in EDX
/// with the board id in ECX; only the low byte of its answer is tested.
///
/// Edge cases: failing fetch, key -1, or empty table -> -1; the search reads
/// `count` words starting at the first entry and returns the zero-based
/// position of the first match.
/// Calling convention: stdcall with one stack argument; the incoming ECX
/// (`this`) is ignored by the original (its first use is the board-id store)
/// and is likewise ignored here.
lf_checker_rt::export!(stdcall, rw_005479c0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xB5;
        const FETCH: u32 = 1;
        const COUNT_SLOT: usize = 1; // frame offset 0x04
        const IDS_SLOT: usize = 2; // frame offset 0x08
        const ROWS_SLOT: usize = 5; // frame offset 0x14
        let mut frame = [0u32; 6];
        let answer =
            lf_checker_rt::callee_fastcall!(FETCH, u32, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return 0xFFFF_FFFF;
        }
        let rows = frame[ROWS_SLOT];
        let key = (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read();
        if key == 0xFFFF_FFFF {
            return 0xFFFF_FFFF;
        }
        let count = frame[COUNT_SLOT];
        if count == 0 {
            return 0xFFFF_FFFF;
        }
        let ids = frame[IDS_SLOT] as *const u32;
        let mut i = 0u32;
        loop {
            if ids.add(i as usize).read() == key {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return 0xFFFF_FFFF;
            }
        }
    }
});
