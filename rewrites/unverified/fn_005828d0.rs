// original: 0x005828D0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_192, player_schema::LeaderboardInfo, 10>::vf8

/// Classify one leaderboard row and map the class to a width.
///
/// Looks the row up exactly like the sibling row lookup (board id in ECX,
/// six-word scratch buffer in EDX, row-table pointer back at buffer word 5),
/// passes the row value in ECX to the row classifier, then maps
/// class-1 through a five-entry table to [4, 8, 8, 0, 4] (anything else, a -1 class,
/// or a failed lookup yields 0). The table values were read off the original
/// at analysis time and are baked in as constants.
///
/// Original: stdcall, one stack word; two direct callees (patched).
lf_checker_rt::export!(stdcall, rw_005828d0(index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x198;
        /// Service-buffer word holding the row-table pointer (byte +0x14).
        const ROWS_WORD: usize = 5;
        let mut lookup: [u32; 6] = [0; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, lookup.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return 0;
        }
        let rows = lookup[ROWS_WORD];
        let row = (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let kind: u32 = lf_checker_rt::callee_thiscall!(2, u32, row);
        if kind == 0xFFFF_FFFF {
            return 0;
        }
        // Row-class table (class-1 -> width), decoded from the original.
        match kind.wrapping_sub(1) {
            0 => 4,
            1 => 8,
            2 => 8,
            3 => 0,
            4 => 4,
            _ => 0,
        }
    }
});
