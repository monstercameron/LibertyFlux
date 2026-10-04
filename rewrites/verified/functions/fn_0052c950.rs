// original: 0x0052C950 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race21Standard, player_schema::LeaderboardInfo, 10>::vf12

/// Look up a leaderboard row's position by its key.
///
/// Asks the leaderboard query helper (id 0x1A) to fill a six-word
/// scratch record (entry count at `+4`, key array at `+8`, id table at
/// `+20`), reads the wanted id from the table at `index`, and returns the
/// first position in the key array holding it. Returns -1 when the query
/// fails, the table holds -1 there, the count is zero, or no key matches.
/// The count comparison is unsigned.
///
/// Original: 0x0052C950 (stdcall, one stack word; incoming ecx ignored).
lf_checker_rt::export!(stdcall, rw_0052C950(index: u32) -> u32 {
    unsafe {
        const LB_ID: u32 = 0x1A;
        const QUERY_CALLEE: u32 = 1;
        const COUNT_SLOT: usize = 1;
        const KEYS_SLOT: usize = 2;
        const TABLE_SLOT: usize = 5;
        const MISSING: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            QUERY_CALLEE, u32, LB_ID, info.as_mut_ptr() as u32
        );
        if ok as u8 == 0 {
            return MISSING;
        }
        let table = info[TABLE_SLOT] as *const u32;
        let want = table.add(index as usize).read_unaligned();
        if want == MISSING {
            return MISSING;
        }
        let count = info[COUNT_SLOT];
        if count == 0 {
            return MISSING;
        }
        let keys = info[KEYS_SLOT] as *const u32;
        let mut i = 0u32;
        loop {
            if keys.add(i as usize).read_unaligned() == want {
                return i;
            }
            i += 1;
            if i >= count {
                return MISSING;
            }
        }
    }
});
