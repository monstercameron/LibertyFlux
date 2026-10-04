// original: 0x00547E90 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_5, player_schema::LeaderboardInfo, 10>::vf13
/// Value lookup by id for one ranked leaderboard board (virtual slot 13).
///
/// Fetches the board tables for board id 0xB7, scans the id array for `id`,
/// and returns the word at the match position from the parallel value array,
/// or -1 when absent.
///
/// Frame layout (offsets from the out-struct pointer): the entry count lands
/// at `+0x0C`, the id-array pointer at `+0x10`, the value-array pointer at
/// `+0x14`. The fetch answer's low byte decides failure (-1). The count is
/// compared signed: zero or negative yields -1. The original re-checks the
/// found index against -1 after the loop, which is unreachable (the index
/// counts up from 0) and is not repeated here; its 8-byte stack alignment is
/// unobservable (only the stack-pointer delta is compared) and likewise not
/// repeated.
/// The shared fetch step fills a six-word frame passed by pointer in EDX
/// with the board id in ECX; only the low byte of its answer is tested.
///
/// Edge cases: failing fetch, non-positive count, or absent id -> -1.
/// Calling convention: stdcall with one stack argument; the incoming ECX
/// (`this`) is ignored by the original (its first use is the board-id store)
/// and is likewise ignored here.
lf_checker_rt::export!(stdcall, rw_00547e90(id: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xB7;
        const FETCH: u32 = 1;
        const COUNT_SLOT: usize = 3; // frame offset 0x0C
        const IDS_SLOT: usize = 4; // frame offset 0x10
        const VALUES_SLOT: usize = 5; // frame offset 0x14
        let mut frame = [0u32; 6];
        let answer =
            lf_checker_rt::callee_fastcall!(FETCH, u32, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return 0xFFFF_FFFF;
        }
        let count = frame[COUNT_SLOT] as i32;
        if count <= 0 {
            return 0xFFFF_FFFF;
        }
        let ids = frame[IDS_SLOT] as *const u32;
        let mut i = 0i32;
        loop {
            if i >= count {
                return 0xFFFF_FFFF;
            }
            if ids.add(i as usize).read() == id {
                break;
            }
            i = i.wrapping_add(1);
        }
        let values = frame[VALUES_SLOT];
        (values.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read()
    }
});
