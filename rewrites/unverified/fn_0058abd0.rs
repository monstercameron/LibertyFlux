// original: 0x0058ABD0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_222, player_schema::LeaderboardInfo, 10>::vf7

/// Fetch one row of this leaderboard's column table by index.
///
/// Asks the schema service (callee id 1, fastcall: board id in ecx, a six-word
/// scratch buffer in edx) for board `0x1b6`. The call answers in al: zero
/// means the board is unknown and the function returns `NONE` (-1). Otherwise
/// the buffer word at `+0x10` holds the row-table pointer and the function
/// returns `table[index]` with 32-bit wrapping address arithmetic, faulting
/// exactly when the original faults for a wild index.
///
/// Original: 0x0058ABD0 (stdcall, one stack word). Incoming ecx is ignored.
lf_checker_rt::export!(stdcall, rw_0058ABD0(index: u32) -> u32 {
    unsafe {
        /// Board id passed to the schema lookup (ecx).
        const BOARD_ID: u32 = 0x1b6;
        /// Intercepted schema-lookup callee.
        const SCHEMA_LOOKUP: u32 = 1;
        /// Buffer word the lookup fills with the row-table pointer ([esp+0x10]).
        const ROW_TABLE_WORD: usize = 4;
        /// Failure sentinel.
        const NONE: u32 = 0xFFFF_FFFF;
        let mut buf = [0u32; 6];
        buf[ROW_TABLE_WORD] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            SCHEMA_LOOKUP, u32, BOARD_ID, buf.as_mut_ptr() as u32
        );
        if (ok & 0xFF) == 0 {
            return NONE;
        }
        let table = buf[ROW_TABLE_WORD];
        (table.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
