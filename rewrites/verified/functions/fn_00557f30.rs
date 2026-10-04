// original: 0x0x00557F30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_36, player_schema::LeaderboardInfo, 10>::vf9

/// Ranked episodic race 36 leaderboard entry-kind query: fetch the
/// board's value table, classify the entry at a caller-supplied index, and
/// map the classifier's answer to a kind code.
///
/// The fetch works as in the sibling column lookup (`BOARD_ID`, buffer word
/// at offset `0x14`, nonzero low byte means success). The table entry becomes
/// the classifier's object argument. Codes 1..=5 map through the original's
/// jump table (1 -> 0, 2 -> 1, 3 -> 3, 4 -> -1, 5 -> 2); anything else,
/// including the classifier's own `-1` and a failed fetch, yields -1.
///
/// The incoming object pointer is ignored. Original: thiscall, one stack
/// word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00557f30(_this: u32, index: u32) -> u32 {
    unsafe {
        const FETCH: u32 = 1;
        const CLASSIFY: u32 = 2;
        const FETCH_WORDS: usize = 6;
        const BOARD_ID: u32 = 0xf0;
        const TABLE_SLOT: usize = 5; // buffer offset 0x14
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut buf = [0u32; FETCH_WORDS];
        let answer: u32 =
            lf_checker_rt::callee_fastcall!(FETCH, u32, BOARD_ID, buf.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return NOT_FOUND;
        }
        let table = buf[TABLE_SLOT];
        let addr = table.wrapping_add(index.wrapping_mul(4));
        let entry = (addr as *const u32).read_unaligned();
        let code: u32 = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, entry);
        match code {
            1 => 0,
            2 => 1,
            3 => 3,
            4 => NOT_FOUND,
            5 => 2,
            _ => NOT_FOUND,
        }
    }
});
