// original: 0x0052C7F0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race20Standard, player_schema::LeaderboardInfo, 10>::vf8

/// Classify a leaderboard row's entry-size class.
///
/// Asks the leaderboard query helper (id 0x14) to fill a six-word
/// scratch record (id table at `+20`), passes the entry at `index` to the
/// row-kind helper, and maps its answer through a five-way jump table.
/// Answers outside 1..=5 and a failed query or -1 kind yield
/// 0.
///
/// Original: 0x0052C7F0 (stdcall, one stack word; incoming ecx ignored).
lf_checker_rt::export!(stdcall, rw_0052C7F0(index: u32) -> u32 {
    unsafe {
        const LB_ID: u32 = 0x14;
        const QUERY_CALLEE: u32 = 1;
        const KIND_CALLEE: u32 = 2;
        const TABLE_SLOT: usize = 5;
        const MISSING: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            QUERY_CALLEE, u32, LB_ID, info.as_mut_ptr() as u32
        );
        if ok as u8 == 0 {
            return 0;
        }
        let table = info[TABLE_SLOT] as *const u32;
        let key = table.add(index as usize).read_unaligned();
        let kind: u32 = lf_checker_rt::callee_thiscall!(KIND_CALLEE, u32, key);
        if kind == MISSING {
            return 0;
        }
        match kind.wrapping_sub(1) {
            0 => 0x00000004,
            1 => 0x00000008,
            2 => 0x00000008,
            3 => 0x00000000,
            4 => 0x00000004,
            _ => 0,
        }
    }
});
