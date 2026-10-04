// original: 0x0054d3a0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_24, player_schema::LeaderboardInfo, 10>::vf6

/// Find the position of a column id in this board's column table.
///
/// `key` is the column id to look for. The board descriptor `BOARD_ID` and a
/// scratch descriptor block are handed to the board-info callee (fastcall:
/// id in `ecx`, block in `edx`), which answers success in `al` and fills the
/// block's entry count (`COUNT_W`, words from the block base) and the
/// column-table pointer (`TABLE_W`). On success the table is scanned for
/// `key` and its index returned. `NOT_FOUND` (-1) is returned when the callee
/// reports failure, when the count is not positive (signed), or when the key
/// is absent; the scan itself is signed (`jl`), matching the original.
///
/// Original: stdcall, one stack word; callee id 1 (fastcall, no stack args).
lf_checker_rt::export!(stdcall, rw_0054d3a0(key: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0xcf;
        const COUNT_W: usize = 3;
        const TABLE_W: usize = 4;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut block = [0u32; 6];
        block[COUNT_W] = 0;
        block[TABLE_W] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD_ID, block.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return NOT_FOUND;
        }
        let count = block[COUNT_W] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let table = block[TABLE_W];
        let mut i = 0i32;
        while i < count {
            let cell = (table.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read_unaligned();
            if cell == key {
                return i as u32;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
