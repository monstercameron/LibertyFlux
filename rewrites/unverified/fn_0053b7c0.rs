// original: 0x0053B7C0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CoopSwatAssault_BG_TIME, player_schema::LeaderboardInfo, 10>::vf7

/// Read one leaderboard row key by position.
///
/// Queries table TABLE_ID; the info descriptor's word 4 points at the
/// row-key array. Returns the key at the argument position, or NOT_FOUND
/// (-1) when the query fails. No bounds check: the position indexes the
/// array with 32-bit wraparound exactly as the original does.
///
/// Original: thiscall with one stack word; ECX (this) is unused.

lf_checker_rt::export!(thiscall, rw_0053b7c0(this: u32, index: u32) -> u32 {
    unsafe {
        const TABLE_ID: u32 = 0x15;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, TABLE_ID, info.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return NOT_FOUND;
        }
        let items = info[4] as *const u32;
        items.wrapping_add(index as usize).read_unaligned()
    }
});
