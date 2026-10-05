// original: 0x0054C280 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_20, player_schema::LeaderboardInfo, 10>::vf7

/// Direct row read for one leaderboard board.
///
/// `slot` is the row requested. Asks the board fetcher (callee 1, fastcall:
/// ECX = board id `BOARD_ID`, EDX = out-block) to fill its out-block: the
/// row array pointer at word `ROWS`. A zero low byte of the answer means no
/// data: return `MISSING`; otherwise return `rows[slot]` with no bounds
/// check, exactly like the original.
///
/// Original: stdcall, one stack word, callee 1 = board fetch, the callee pops 4 bytes.
lf_checker_rt::export!(stdcall, rw_0054C280(slot: u32) -> u32 {    unsafe {
        const BOARD_ID: u32 = 0xcb;
        const FETCH: u32 = 1;
        const ROWS: usize = 4;
        const MISSING: u32 = 0xFFFF_FFFF;
        let mut out = [0u32; 5];
        let ok: u32 = lf_checker_rt::callee_fastcall!(FETCH, u32, BOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return MISSING;
        }
        ((out[ROWS].wrapping_add(slot.wrapping_mul(4))) as *const u32).read_unaligned()
    }});
