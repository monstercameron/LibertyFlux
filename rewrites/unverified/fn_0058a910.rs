// original: 0x0058A910 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_222, player_schema::LeaderboardInfo, 10>::vf12

/// Map a row index to a key position in this leaderboard's tables.
///
/// Asks the schema service (callee id 1, fastcall: board id in ecx, scratch
/// buffer in edx) for board `0x1b6`. The call answers in al; on zero the
/// function returns `NONE` (-1). Otherwise it reads the key `rows[index]`
/// (row-table pointer at buffer `+0x14`) and rejects the `NONE` sentinel.
/// Then it scans the key table (buffer `+0x08`) while `i < count` (buffer
/// `+0x04`, unsigned; an empty table returns `NONE`) and returns the first
/// position holding the key, or `NONE` on a miss.
///
/// Original: 0x0058A910 (stdcall, one stack word). Incoming ecx is ignored.
lf_checker_rt::export!(stdcall, rw_0058A910(index: u32) -> u32 {
    unsafe {
        /// Board id passed to the schema lookup (ecx).
        const BOARD_ID: u32 = 0x1b6;
        /// Intercepted schema-lookup callee.
        const SCHEMA_LOOKUP: u32 = 1;
        /// Buffer word holding the entry count ([esp+0x08]).
        const COUNT_WORD: usize = 1;
        /// Buffer word holding the key-table pointer ([esp+0x0c]).
        const KEYS_WORD: usize = 2;
        /// Buffer word holding the row-table pointer ([esp+0x18]).
        const ROWS_WORD: usize = 5;
        /// Failure sentinel.
        const NONE: u32 = 0xFFFF_FFFF;
        let mut buf = [0u32; 6];
        buf[COUNT_WORD] = 0;
        buf[KEYS_WORD] = 0;
        buf[ROWS_WORD] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            SCHEMA_LOOKUP, u32, BOARD_ID, buf.as_mut_ptr() as u32
        );
        if (ok & 0xFF) == 0 {
            return NONE;
        }
        let rows = buf[ROWS_WORD];
        let key = (rows.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        if key == NONE {
            return NONE;
        }
        let count = buf[COUNT_WORD];
        if count == 0 {
            return NONE;
        }
        let keys = buf[KEYS_WORD];
        let mut i: u32 = 0;
        while i < count {
            let v = (keys.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if v == key {
                return i;
            }
            i += 1;
        }
        NONE
    }
});
