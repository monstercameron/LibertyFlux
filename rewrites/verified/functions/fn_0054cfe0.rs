// original: 0x0054CFE0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_23, player_schema::LeaderboardInfo, 10>::vf8

/// Row-kind classifier for one leaderboard board.
///
/// `slot` is the row requested. Asks the board fetcher (callee 1, fastcall:
/// ECX = board id `BOARD_ID`, EDX = out-block) to fill its out-block: the
/// row array pointer at word `ROWS`. A zero low byte of the answer means no
/// data: return 0.
///
/// Otherwise passes `rows[slot]` to the kind probe (callee 2, thiscall:
/// ECX = row value). A `MISSING` probe answer returns 0; otherwise the
/// answer minus one selects from the five-way table `CLASS_OF` (answers
/// outside 1..=5 return 0).
///
/// Original: stdcall, one stack word, callees 1 (fetch) and 2 (kind), the callee pops 4 bytes.
lf_checker_rt::export!(stdcall, rw_0054CFE0(slot: u32) -> u32 {    unsafe {
        const BOARD_ID: u32 = 0xce;
        const FETCH: u32 = 1;
        const KIND: u32 = 2;
        const ROWS: usize = 5;
        const MISSING: u32 = 0xFFFF_FFFF;
        const CLASS_OF: [u32; 5] = [4, 8, 8, 0, 4];
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(FETCH, u32, BOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return 0;
        }
        let row = ((out[ROWS].wrapping_add(slot.wrapping_mul(4))) as *const u32).read_unaligned();
        let kind: u32 = lf_checker_rt::callee_thiscall!(KIND, u32, row);
        if kind == MISSING {
            return 0;
        }
        CLASS_OF.get(kind.wrapping_sub(1) as usize).copied().unwrap_or(0)
    }});
