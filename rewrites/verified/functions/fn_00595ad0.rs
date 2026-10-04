// original: 0x00595AD0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_262, player_schema::LeaderboardInfo, 10>::vf7
/// Ranked-leaderboard column lookup (vf7 slot, race 262).
///
/// Asks the leaderboard helper for the column table of leaderboard 0x1da
/// and returns `table[index]`; returns `u32::MAX` when the helper reports
/// failure. Only the low byte of the helper answer is significant.
export!(stdcall, rw_00595ad0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1da;
        // Out-words the helper fills; the table pointer lands at word 4,
        // matching the original's frame layout relative to its own buffer.
        let mut out = [0u32; 5];
        let ok: u32 = callee_fastcall!(1, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32);
        if ok & 0xff == 0 {
            return u32::MAX;
        }
        let table = out[4];
        core::ptr::read(table.wrapping_add(index.wrapping_mul(4)) as *const u32)
    }
});
