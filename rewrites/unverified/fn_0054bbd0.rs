// original: 0x0054bbd0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_19, player_schema::LeaderboardInfo, 10>::vf13

/// Translate a column id through the board into its row-table entry.
///
/// `key` is looked up in the column table and the row-table entry at the
/// same position is returned. The board descriptor `BOARD_ID` and a scratch
/// block go to the board-info callee (fastcall: id in `ecx`, block in `edx`,
/// id 1), which answers success in `al` and fills the block's entry count
/// (`COUNT_W`), column-table pointer (`COLS_W`) and row-table pointer
/// (`ROWS_W`, all words from the block base). A non-positive count (signed)
/// or an absent key gives `NOT_FOUND` (-1); the scan is signed (`jl`).
/// (The original re-checks the found index against -1 after the scan; that
/// branch is dead, since a found index is never negative, and is omitted.)
///
/// Original: stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_0054bbd0(key: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0xca;
        const COUNT_W: usize = 3;
        const COLS_W: usize = 4;
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
        let count = block[COUNT_W] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let cols = block[COLS_W];
        let mut i = 0i32;
        let at = loop {
            let cell = (cols.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read_unaligned();
            if cell == key {
                break i as u32;
            }
            i += 1;
            if i >= count {
                return NOT_FOUND;
            }
        };
        let rows = block[ROWS_W];
        (rows.wrapping_add(at.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
