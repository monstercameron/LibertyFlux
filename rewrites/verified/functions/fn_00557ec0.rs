// original: 0x0x00557EC0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_36, player_schema::LeaderboardInfo, 10>::vf8

/// Ranked episodic race 36 leaderboard entry-size query: fetch the
/// board's value table, classify the entry at a caller-supplied index, and
/// map the classifier's answer to a byte size.
///
/// The fetch works as in the sibling column lookup (`BOARD_ID`, buffer word
/// at offset `0x14`, nonzero low byte means success). The table entry becomes
/// the classifier's object argument. The classifier answers an integer code;
/// codes 1..=5 map to sizes through the original's jump table (1 -> 4,
/// 2 -> 8, 3 -> 8, 4 -> 0, 5 -> 4) and anything else, including the
/// classifier's own `-1` and a failed fetch, yields 0.
///
/// The incoming object pointer is ignored. Original: thiscall, one stack
/// word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00557ec0(_this: u32, index: u32) -> u32 {
    unsafe {
        const FETCH: u32 = 1;
        const CLASSIFY: u32 = 2;
        const FETCH_WORDS: usize = 6;
        const BOARD_ID: u32 = 0xf0;
        const TABLE_SLOT: usize = 5; // buffer offset 0x14
        let mut buf = [0u32; FETCH_WORDS];
        let answer: u32 =
            lf_checker_rt::callee_fastcall!(FETCH, u32, BOARD_ID, buf.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return 0;
        }
        let table = buf[TABLE_SLOT];
        let addr = table.wrapping_add(index.wrapping_mul(4));
        let entry = (addr as *const u32).read_unaligned();
        let code: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, entry);
        match code {
            1 => 4,
            2 | 3 => 8,
            4 => 0,
            5 => 4,
            _ => 0,
        }
    }
});
