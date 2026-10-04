// original: 0x00547400 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_2, player_schema::LeaderboardInfo, 10>::vf8
/// Column-width lookup for one ranked leaderboard board (virtual slot 8).
///
/// Fetches the board tables for board id 0xA0, classifies the key word at
/// `index` (`keys[index]`) through the shared classify step, and maps the
/// class to a width: class 1 -> 4, classes 2-3 -> 8, class 4 -> 0,
/// class 5 -> 4, anything else -> 0. The mapping is the original's five-entry
/// jump table, read from the image and encoded as a match.
///
/// Frame layout: the keys-table pointer lands at `+0x14` of the out-struct.
/// The fetch answer's low byte decides failure (result 0); a classify answer
/// of -1 also yields 0, and the table index is the answer minus one, so class
/// 0 wraps out of range and yields 0 as well.
/// The shared fetch step fills a six-word frame passed by pointer in EDX
/// with the board id in ECX; only the low byte of its answer is tested.
///
/// Edge cases: failing fetch -> 0 without touching the tables; wild index
/// faults on the same single load as the original.
/// Calling convention: stdcall with one stack argument; the incoming ECX
/// (`this`) is ignored by the original (its first use is the board-id store)
/// and is likewise ignored here.
lf_checker_rt::export!(stdcall, rw_00547400(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xA0;
        const FETCH: u32 = 1;
        const CLASSIFY: u32 = 2;
        const KEYS_SLOT: usize = 5; // frame offset 0x14
        let mut frame = [0u32; 6];
        let answer =
            lf_checker_rt::callee_fastcall!(FETCH, u32, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return 0;
        }
        let keys = frame[KEYS_SLOT];
        let key = (keys.wrapping_add(index.wrapping_mul(4)) as *const u32).read();
        let class = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, key);
        if class == 0xFFFF_FFFF {
            return 0;
        }
        match class.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            3 => 0,
            4 => 4,
            _ => 0,
        }
    }
});
