// original: 0x0054c790 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_21, player_schema::LeaderboardInfo, 10>::vf9

/// Map the column id at position `index` to its rank code.
///
/// Same shape as the sibling width classifier: the board-info callee
/// (fastcall: `BOARD_ID` in `ecx`, scratch block in `edx`, id 1) answers
/// success in `al` and fills the block's column-table pointer (`TABLE_W`,
/// words from the block base); the table entry at `index` goes (in `ecx`)
/// to the classifier callee (id 2, thiscall, no stack args) whose answer
/// `r` selects the result: `r == -1` or `r - 1 > 4` gives -1, else
/// `RANK[r - 1]` with `RANK = [0, 1, 3, -1, 2]` (the original's jump table,
/// decoded during analysis; note positions 2 and 4 are swapped relative to
/// the identity). A callee-1 failure also gives -1.
///
/// Original: stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_0054c790(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0xcc;
        const TABLE_W: usize = 5;
        const RANK: [u32; 5] = [0, 1, 3, 0xFFFF_FFFF, 2];
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        let mut block = [0u32; 6];
        block[TABLE_W] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD_ID, block.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return NOT_FOUND;
        }
        let table = block[TABLE_W];
        let entry = (table.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let r: u32 = lf_checker_rt::callee_thiscall!(2, u32, entry);
        if r == 0xFFFF_FFFF {
            return NOT_FOUND;
        }
        let i = r.wrapping_sub(1);
        if i > 4 {
            return NOT_FOUND;
        }
        RANK[i as usize]
    }
});
