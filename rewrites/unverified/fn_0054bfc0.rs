// original: 0x0054bfc0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_20, player_schema::LeaderboardInfo, 10>::vf12

/// Map a row position to its rank: look the row's key up in the column table.
///
/// `pos` selects a row. The board descriptor `BOARD_ID` and a scratch block
/// go to the board-info callee (fastcall: id in `ecx`, block in `edx`, id
/// 1), which answers success in `al` and fills the block's entry count
/// (`COUNT_W`), column-table pointer (`COLS_W`) and row-table pointer
/// (`ROWS_W`, all words from the block base). The key at `pos` in the row
/// table is read first: a key of -1 gives `NOT_FOUND` at once. Otherwise
/// the column table is scanned for the key (unsigned scan, `jb`) and its
/// index returned; an empty table or an absent key gives `NOT_FOUND`.
///
/// Original: stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_0054bfc0(pos: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0xcb;
        const COUNT_W: usize = 1;
        const COLS_W: usize = 2;
        const ROWS_W: usize = 5;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut block = [0u32; 6];
        block[COUNT_W] = 0;
        block[COLS_W] = 0;
        block[ROWS_W] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD_ID, block.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return NOT_FOUND;
        }
        let rows = block[ROWS_W];
        let key = (rows.wrapping_add(pos.wrapping_mul(4)) as *const u32).read_unaligned();
        if key == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = block[COUNT_W];
        if count == 0 {
            return NOT_FOUND;
        }
        let cols = block[COLS_W];
        let mut i = 0u32;
        loop {
            let cell = (cols.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if cell == key {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});
