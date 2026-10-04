// original: 0x0054cfe0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_23, player_schema::LeaderboardInfo, 10>::vf8

/// Classify the column id at position `index` into a width class.
///
/// The board descriptor `BOARD_ID` and a scratch block go to the board-info
/// callee (fastcall: id in `ecx`, block in `edx`), which answers success in
/// `al` and fills the block's column-table pointer (`TABLE_W`, words from
/// the block base). The table entry at `index` is then passed (in `ecx`) to
/// the classifier callee (id 2, thiscall, no stack args) whose answer `r`
/// selects the result: `r == -1` or `r - 1 > 4` gives 0, else
/// `CLASS[r - 1]` with `CLASS = [4, 8, 8, 0, 4]` (the original's jump table,
/// decoded during analysis). A callee-1 failure also gives 0.
///
/// Original: stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_0054cfe0(index: u32) -> u32 {
    unsafe {
        const BOARD_ID: u32 = 0xce;
        const TABLE_W: usize = 5;
        const CLASS: [u32; 5] = [4, 8, 8, 0, 4];
        let mut block = [0u32; 6];
        block[TABLE_W] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD_ID, block.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return 0;
        }
        let table = block[TABLE_W];
        let entry = (table.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        let r: u32 = lf_checker_rt::callee_thiscall!(2, u32, entry);
        if r == 0xFFFF_FFFF {
            return 0;
        }
        let i = r.wrapping_sub(1);
        if i > 4 {
            return 0;
        }
        CLASS[i as usize]
    }
});
