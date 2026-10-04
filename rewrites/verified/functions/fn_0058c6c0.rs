// original: 0x0058C6C0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_228, player_schema::LeaderboardInfo, 10>::vf9
/// Leaderboard grade rank: rank of element INDEX, or -1.
///
/// Arguments: INDEX is the single stack word. ECX is ignored. Calls the
/// shared lookup helper (fastcall: ECX = board id 0x1BC, EDX = frame
/// buffer; AL tested), reads the array pointer at buffer+0x14, loads
/// array[INDEX] with no bounds check, and passes it in ECX to the grade
/// helper. If the grade is -1, or grade-1 is above 4 (unsigned), returns
/// 0xFFFFFFFF. Otherwise maps grade-1 through a 5-entry table to [0, 1, 3, -1, 2]:
/// 0->0, 1->1, 2->3, 3->-1, 4->2. AL == 0 also returns 0xFFFFFFFF.
/// Original: 0x0058C6C0 (stdcall, one stack word; ignores ECX).

lf_checker_rt::export!(stdcall, rw_0058C6C0(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0x1BC;
        const ARRAY_OFF: u32 = 0x14;
        const NONE: u32 = 0xFFFFFFFF;
        let mut out = [0u32; 8];
        let r: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD_ID, out.as_mut_ptr() as u32);
        if (r & 0xFF) == 0 {
            return NONE;
        }
        let arr = out[(ARRAY_OFF / 4) as usize];
        let v = ((arr.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned();
        let g: u32 = lf_checker_rt::callee_thiscall!(2, u32, v);
        if g == NONE {
            return NONE;
        }
        match g.wrapping_sub(1) {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => NONE,
            4 => 2,
            _ => NONE,
        }
    }
});
