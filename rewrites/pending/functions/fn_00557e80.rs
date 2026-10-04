// original: 0x0x00557E80 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_36, player_schema::LeaderboardInfo, 10>::vf7

/// Ranked episodic race 36 leaderboard column lookup: fetch the board's
/// value table, then return the entry at a caller-supplied index.
///
/// `index` is a zero-based slot in the table the fetch helper returns for
/// this board (`BOARD_ID`). The helper is asked for the table through a
/// callee-filled buffer (`TABLE_SLOT` is the word at buffer offset `0x10`)
/// and answers success in the low byte of its return value; any nonzero low
/// byte means success. On fetch failure, or when the entry address itself is
/// unreadable, the result is `NOT_FOUND` (`-1`); a fault on the unreadable
/// address is the original's behaviour too and is compared as fault parity.
///
/// The incoming object pointer is ignored: this slot reads no object state.
/// Original: thiscall, one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00557e80(_this: u32, index: u32) -> u32 {
    unsafe {
        const FETCH: u32 = 1;
        const FETCH_WORDS: usize = 6;
        const BOARD_ID: u32 = 0xf0;
        const TABLE_SLOT: usize = 4; // buffer offset 0x10
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut buf = [0u32; FETCH_WORDS];
        let answer: u32 =
            lf_checker_rt::callee_fastcall!(FETCH, u32, BOARD_ID, buf.as_mut_ptr() as u32);
        if answer & 0xFF == 0 {
            return NOT_FOUND;
        }
        let table = buf[TABLE_SLOT];
        let addr = table.wrapping_add(index.wrapping_mul(4));
        (addr as *const u32).read_unaligned()
    }
});
