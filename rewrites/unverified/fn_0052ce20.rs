// original: 0x0052CE20 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race22Standard, player_schema::LeaderboardInfo, 10>::vf13

/// Map a leaderboard key to its value.
///
/// Asks the leaderboard query helper (id 0x1B) to fill a six-word
/// scratch record (signed entry count at `+12`, key array at `+16`, value
/// array at `+20`) and returns the value at the first position whose key
/// equals `want`. Returns -1 when the query fails, the count is zero or
/// negative, or no key matches. The redundant index check after the loop
/// can never fire (the loop only exits with a valid index or -1 already
/// returned) and is kept for fidelity.
///
/// Original: 0x0052CE20 (stdcall, one stack word; incoming ecx ignored).
lf_checker_rt::export!(stdcall, rw_0052CE20(want: u32) -> u32 {
    unsafe {
        const LB_ID: u32 = 0x1B;
        const QUERY_CALLEE: u32 = 1;
        const COUNT_SLOT: usize = 3;
        const KEYS_SLOT: usize = 4;
        const VALS_SLOT: usize = 5;
        const MISSING: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            QUERY_CALLEE, u32, LB_ID, info.as_mut_ptr() as u32
        );
        if ok as u8 == 0 {
            return MISSING;
        }
        let count = info[COUNT_SLOT] as i32;
        if count <= 0 {
            return MISSING;
        }
        let keys = info[KEYS_SLOT] as *const u32;
        let mut i = 0i32;
        loop {
            if keys.add(i as usize).read_unaligned() == want {
                break;
            }
            i += 1;
            if i >= count {
                return MISSING;
            }
        }
        if i == -1 {
            return MISSING;
        }
        let vals = info[VALS_SLOT] as *const u32;
        vals.add(i as usize).read_unaligned()
    }
});
