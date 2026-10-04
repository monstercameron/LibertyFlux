// original: 0x0058B070 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_223, player_schema::LeaderboardInfo, 10>::vf8

/// Size in bytes of one cell of this leaderboard's column table.
///
/// Asks the schema service (callee id 1, fastcall: board id in ecx, scratch
/// buffer in edx) for board `0x1b7`. The call answers in al; on zero the
/// function returns 0. Otherwise it reads the cell `rows[index]` (row-table
/// pointer at buffer `+0x14`) and asks the column-kind service (callee id 2,
/// thiscall, cell in ecx) for its kind. Kind -1, 0 and anything above 5 give
/// 0; kinds 1 and 5 give 4, kinds 2 and 3 give 8, kind 4 gives 0 (a five-way
/// jump table over `kind - 1` in the original).
///
/// Original: 0x0058B070 (stdcall, one stack word). Incoming ecx is ignored.
lf_checker_rt::export!(stdcall, rw_0058B070(index: u32) -> u32 {
    unsafe {
        /// Board id passed to the schema lookup (ecx).
        const BOARD_ID: u32 = 0x1b7;
        /// Intercepted schema-lookup callee.
        const SCHEMA_LOOKUP: u32 = 1;
        /// Intercepted column-kind callee.
        const COLUMN_KIND: u32 = 2;
        /// Buffer word the lookup fills with the row-table pointer ([esp+0x14]).
        const ROWS_WORD: usize = 5;
        /// Failure sentinel of the kind service.
        const NO_KIND: u32 = 0xFFFF_FFFF;
        let mut buf = [0u32; 6];
        buf[ROWS_WORD] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            SCHEMA_LOOKUP, u32, BOARD_ID, buf.as_mut_ptr() as u32
        );
        if (ok & 0xFF) == 0 {
            return 0;
        }
        let rows = buf[ROWS_WORD];
        let cell = (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let kind: u32 = lf_checker_rt::callee_thiscall!(COLUMN_KIND, u32, cell);
        if kind == NO_KIND {
            return 0;
        }
        match kind.wrapping_sub(1) {
            0 => 4,
            1 | 2 => 8,
            4 => 4,
            _ => 0,
        }
    }
});
