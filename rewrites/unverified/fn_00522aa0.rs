// original: 0x00522aa0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race44NoHolds, player_schema::LeaderboardInfo, 10>::vf8

/// Map one leaderboard entry through the rank-class table.
///
/// `value_index` selects an entry of the value list. The lookup callee
/// (id 1, leaderboard id 0x19) fills the value-list pointer at word 5 of a
/// six-word query block; only its low result byte matters. The entry is
/// passed to the classifier callee (id 2); its answer minus one indexes a
/// five-entry table (answers 1 to 5 map to 4, 8, 8, 0, 4), anything else yields
/// zero, as does a failed lookup.
/// Edge cases: classifier answers -1, 0 and above 5 all yield zero; the
/// subtraction and the bound check are unsigned.
/// Original: stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_00522aa0(value_index: u32) -> u32 {
    unsafe {
        const LOOKUP_CALLEE: u32 = 1;
        const MAP_CALLEE: u32 = 2;
        const LEADERBOARD_ID: u32 = 0x19;
        const DEFAULT: u32 = 0;

        let mut block = [0u32; 6];
        let answer: u32 = lf_checker_rt::callee_fastcall!(
            LOOKUP_CALLEE,
            u32,
            LEADERBOARD_ID,
            block.as_mut_ptr() as u32
        );
        if answer & 0xFF == 0 {
            return DEFAULT;
        }
        let element = (block[5].wrapping_add(value_index.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let s: u32 = lf_checker_rt::callee_thiscall!(MAP_CALLEE, u32, element);
        if s == 0xFFFF_FFFF {
            return DEFAULT;
        }
        let d = s.wrapping_sub(1);
        if d > 4 {
            return DEFAULT;
        }
        match d {
            0 => 4,
            1 => 8,
            2 => 8,
            3 => 0,
            _ => 4,
        }
    }
});
