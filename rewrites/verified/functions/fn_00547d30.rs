// original: 0x00547D30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_4, player_schema::LeaderboardInfo, 10>::vf9
/// Column-kind lookup for one ranked leaderboard board (virtual slot 9).
///
/// Same shape as the slot-8 width lookup for board id 0xB5, with a different
/// class mapping: class 1 -> 0, class 2 -> 1, class 3 -> 3, class 4 -> -1,
/// class 5 -> 2, anything else -> -1 (the original's five-entry jump table,
/// read from the image and encoded as a match; note class 3 maps to 3, not 2).
///
/// Frame layout: the keys-table pointer lands at `+0x14` of the out-struct.
/// A failing fetch (low answer byte zero) or a classify answer of -1 yields
/// -1; the table index is the answer minus one, so class 0 wraps out of range
/// and yields -1 as well.
/// The shared fetch step fills a six-word frame passed by pointer in EDX
/// with the board id in ECX; only the low byte of its answer is tested.
///
/// Edge cases: failing fetch -> -1 without touching the tables; wild index
/// faults on the same single load as the original.
/// Calling convention: stdcall with one stack argument; the incoming ECX
/// (`this`) is ignored by the original (its first use is the board-id store)
/// and is likewise ignored here.
lf_checker_rt::export!(stdcall, rw_00547d30(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0xB5;
        const FETCH: u32 = 1;
        const CLASSIFY: u32 = 2;
        const KEYS_SLOT: usize = 5; // frame offset 0x14
        let mut frame = [0u32; 6];
        let answer =
            lf_checker_rt::callee_fastcall!(FETCH, u32, LEADERBOARD_ID, frame.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return 0xFFFF_FFFF;
        }
        let keys = frame[KEYS_SLOT];
        let key = (keys.wrapping_add(index.wrapping_mul(4)) as *const u32).read();
        let class = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, key);
        if class == 0xFFFF_FFFF {
            return 0xFFFF_FFFF;
        }
        match class.wrapping_sub(1) {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => 0xFFFF_FFFF,
            4 => 2,
            _ => 0xFFFF_FFFF,
        }
    }
});
