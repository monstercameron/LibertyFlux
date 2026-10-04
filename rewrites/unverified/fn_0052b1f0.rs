// original: 0x0052B1F0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race15Standard, player_schema::LeaderboardInfo, 10>::vf7

/// Read one leaderboard table entry by index.
///
/// Asks the leaderboard query helper (id 0x5D) to fill a six-word
/// scratch record (id table at `+16`) and returns the entry at `index`.
/// Returns -1 when the query fails. The index is used unchecked, so only
/// in-range indices are exercised.
///
/// Original: 0x0052B1F0 (stdcall, one stack word; incoming ecx ignored).
lf_checker_rt::export!(stdcall, rw_0052B1F0(index: u32) -> u32 {
    unsafe {
        const LB_ID: u32 = 0x5D;
        const QUERY_CALLEE: u32 = 1;
        const TABLE_SLOT: usize = 4;
        const MISSING: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            QUERY_CALLEE, u32, LB_ID, info.as_mut_ptr() as u32
        );
        if ok as u8 == 0 {
            return MISSING;
        }
        let table = info[TABLE_SLOT] as *const u32;
        table.add(index as usize).read_unaligned()
    }
});
