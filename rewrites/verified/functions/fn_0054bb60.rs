// original: 0x0054BB60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_19, player_schema::LeaderboardInfo, 10>::vf12

/// Rank lookup for one leaderboard board: map a row slot to its rank index.
///
/// `slot` is the row requested. Asks the board fetcher (callee 1, fastcall:
/// ECX = board id `BOARD_ID`, EDX = out-block) to fill its out-block: entry
/// count at word `COUNT`, key array pointer at `KEYS`, value array pointer at
/// `VALS`. A zero low byte of the answer means no data: return `MISSING`.
///
/// Otherwise read the key `values[slot]`; a `MISSING` key or an empty table
/// also returns `MISSING`. Scan `keys[0..count]` (unsigned bound) for the
/// first slot holding the key and return its index, or `MISSING` when absent.
///
/// Original: stdcall, one stack word, callee 1 = board fetch, the callee pops 4 bytes.
lf_checker_rt::export!(stdcall, rw_0054BB60(slot: u32) -> u32 {    unsafe {
        const BOARD_ID: u32 = 0xca;
        const FETCH: u32 = 1;
        const COUNT: usize = 1;
        const KEYS: usize = 2;
        const VALS: usize = 5;
        const MISSING: u32 = 0xFFFF_FFFF;
        let mut out = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(FETCH, u32, BOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return MISSING;
        }
        let key = ((out[VALS].wrapping_add(slot.wrapping_mul(4))) as *const u32).read_unaligned();
        if key == MISSING {
            return MISSING;
        }
        let count = out[COUNT];
        if count == 0 {
            return MISSING;
        }
        let keys = out[KEYS];
        let mut i = 0u32;
        loop {
            let cand = ((keys.wrapping_add(i.wrapping_mul(4))) as *const u32).read_unaligned();
            if cand == key {
                return i;
            }
            i += 1;
            if i >= count {
                return MISSING;
            }
        }
    }});
