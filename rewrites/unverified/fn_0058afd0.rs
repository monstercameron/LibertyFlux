// original: 0x0058AFD0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_223, player_schema::LeaderboardInfo, 10>::vf6

/// Find a value in this leaderboard's key column (signed linear search).
///
/// Asks the schema service (callee id 1, fastcall: board id in ecx, scratch
/// buffer in edx) for board `0x1b7`. The call answers in al; on zero, or
/// when the returned count (buffer `+0x0c`, read signed) is not positive, the
/// function returns `NONE` (-1). Otherwise it scans the key table (buffer
/// `+0x10`) while `i < count` (signed) and returns the first position holding
/// `want`, or `NONE` when no entry matches.
///
/// Original: 0x0058AFD0 (stdcall, one stack word). Incoming ecx is ignored.
lf_checker_rt::export!(stdcall, rw_0058AFD0(want: u32) -> u32 {
    unsafe {
        /// Board id passed to the schema lookup (ecx).
        const BOARD_ID: u32 = 0x1b7;
        /// Intercepted schema-lookup callee.
        const SCHEMA_LOOKUP: u32 = 1;
        /// Buffer word holding the entry count ([esp+0x14]).
        const COUNT_WORD: usize = 3;
        /// Buffer word holding the key-table pointer ([esp+0x18]).
        const KEYS_WORD: usize = 4;
        /// Failure sentinel.
        const NONE: u32 = 0xFFFF_FFFF;
        let mut buf = [0u32; 6];
        buf[COUNT_WORD] = 0;
        buf[KEYS_WORD] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            SCHEMA_LOOKUP, u32, BOARD_ID, buf.as_mut_ptr() as u32
        );
        if (ok & 0xFF) == 0 {
            return NONE;
        }
        let count = buf[COUNT_WORD] as i32;
        if count <= 0 {
            return NONE;
        }
        let keys = buf[KEYS_WORD];
        let mut i: i32 = 0;
        while i < count {
            let v = (keys.wrapping_add((i as u32).wrapping_mul(4)) as *const u32)
                .read_unaligned();
            if v == want {
                return i as u32;
            }
            i += 1;
        }
        NONE
    }
});
