// original: 0x0054CD50 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_23, player_schema::LeaderboardInfo, 10>::vf13

/// Row-key search with value mapping for one leaderboard board.
///
/// `want` is the key sought. Asks the board fetcher (callee 1, fastcall:
/// ECX = board id `BOARD_ID`, EDX = out-block) to fill its out-block: entry
/// count at word `COUNT`, key array pointer at `KEYS`, mapped-value array
/// pointer at `VALS`. A zero low byte of the answer means no data: `MISSING`.
///
/// A non-positive count (signed) also returns `MISSING`. Otherwise scan
/// `keys[0..count]` for `want` and return the mapped value at the first hit,
/// or `MISSING` when absent. The original re-checks the hit index against -1
/// after the loop; an index in `0..count` can never be -1, so the check is dead
/// and not reproduced.
///
/// Original: stdcall, one stack word, callee 1 = board fetch, the callee pops 4 bytes.
lf_checker_rt::export!(stdcall, rw_0054CD50(want: u32) -> u32 {    unsafe {
        const BOARD_ID: u32 = 0xce;
        const FETCH: u32 = 1;
        const COUNT: usize = 3;
        const KEYS: usize = 4;
        const VALS: usize = 5;
        const MISSING: u32 = 0xFFFF_FFFF;
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(FETCH, u32, BOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return MISSING;
        }
        let count = out[COUNT] as i32;
        if count <= 0 {
            return MISSING;
        }
        let keys = out[KEYS];
        let vals = out[VALS];
        let mut i = 0u32;
        loop {
            let cand = ((keys.wrapping_add(i.wrapping_mul(4))) as *const u32).read_unaligned();
            if cand == want {
                return ((vals.wrapping_add(i.wrapping_mul(4))) as *const u32).read_unaligned();
            }
            i += 1;
            if (i as i32) >= count {
                return MISSING;
            }
        }
    }});
