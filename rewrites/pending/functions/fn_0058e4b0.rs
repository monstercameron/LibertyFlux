// original: 0x0058E4B0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_235, player_schema::LeaderboardInfo, 10>::vf7
/// Leaderboard column read: element INDEX of this board's value array, or -1.
///
/// Arguments: INDEX is the single stack word. ECX is ignored (overwritten
/// with the board id). Calls the shared lookup helper (fastcall: ECX = board
/// id 0x1C3, EDX = frame buffer) and tests only AL. On success the array
/// pointer is at buffer+0x10 and the result is array[INDEX] with no bounds
/// check, so a wild index faults exactly like the original. AL == 0 returns
/// 0xFFFFFFFF without reading memory.
/// Original: 0x0058E4B0 (stdcall, one stack word; ignores ECX).

lf_checker_rt::export!(stdcall, rw_0058E4B0(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x1C3;
        const ARRAY_OFF: u32 = 0x10;
        let mut out = [0u32; 8];
        let r: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD_ID, out.as_mut_ptr() as u32);
        if (r & 0xFF) == 0 {
            return 0xFFFFFFFF;
        }
        let arr = out[(ARRAY_OFF / 4) as usize];
        ((arr.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned()
    }
});
