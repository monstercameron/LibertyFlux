// original: 0x0058A980 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_222, player_schema::LeaderboardInfo, 10>::vf13

/// Look up a value by key in this leaderboard's column tables.
///
/// Asks the schema service (callee id 1, fastcall: board id in ecx, scratch
/// buffer in edx) for board `0x1b6`. The call answers in al; on zero, or
/// when the returned count (buffer `+0x0c`, read signed) is not positive, the
/// function returns `NONE` (-1). Otherwise it scans the key table (buffer
/// `+0x10`) while `i < count` (signed) for `want` and returns the value-table
/// entry (buffer `+0x14`) at the matching position, or `NONE` on a miss. The
/// original re-tests the found position against -1 after the loop; that test
/// can never fire (positions start at 0) and is not reproduced.
///
/// Original: 0x0058A980 (stdcall, one stack word). Incoming ecx is ignored.
lf_checker_rt::export!(stdcall, rw_0058A980(want: u32) -> u32 {
    unsafe {
        /// Board id passed to the schema lookup (ecx).
        const BOARD_ID: u32 = 0x1b6;
        /// Intercepted schema-lookup callee.
        const SCHEMA_LOOKUP: u32 = 1;
        /// Buffer word holding the entry count ([esp+0x14]).
        const COUNT_WORD: usize = 3;
        /// Buffer word holding the key-table pointer ([esp+0x18]).
        const KEYS_WORD: usize = 4;
        /// Buffer word holding the value-table pointer ([esp+0x1c]).
        const VALS_WORD: usize = 5;
        /// Failure sentinel.
        const NONE: u32 = 0xFFFF_FFFF;
        let mut buf = [0u32; 6];
        buf[COUNT_WORD] = 0;
        buf[KEYS_WORD] = 0;
        buf[VALS_WORD] = 0;
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
        let vals = buf[VALS_WORD];
        let mut i: i32 = 0;
        while i < count {
            let v = (keys.wrapping_add((i as u32).wrapping_mul(4)) as *const u32)
                .read_unaligned();
            if v == want {
                return (vals.wrapping_add((i as u32).wrapping_mul(4)) as *const u32)
                    .read_unaligned();
            }
            i += 1;
        }
        NONE
    }
});
