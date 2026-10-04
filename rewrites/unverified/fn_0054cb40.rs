// original: 0x0054cb40 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_22, player_schema::LeaderboardInfo, 10>::vf7

/// Read one entry of this board's column table by position.
///
/// `index` selects the entry. The board descriptor `BOARD_ID` and a scratch
/// descriptor block go to the board-info callee (fastcall: id in `ecx`,
/// block in `edx`), which answers success in `al` and fills the block's
/// column-table pointer (`TABLE_W`, words from the block base). On success
/// the entry at `index` is returned; there is no bounds check, so an index
/// past the table reads whatever follows it. `NOT_FOUND` (-1) is returned
/// only when the callee reports failure.
///
/// Original: stdcall, one stack word; callee id 1 (fastcall, no stack args).
lf_checker_rt::export!(stdcall, rw_0054cb40(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0xcd;
        const TABLE_W: usize = 4;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut block = [0u32; 5];
        block[TABLE_W] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD_ID, block.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return NOT_FOUND;
        }
        let table = block[TABLE_W];
        (table.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
