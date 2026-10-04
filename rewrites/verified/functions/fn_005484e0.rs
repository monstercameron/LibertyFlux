// original: 0x005484E0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_6, player_schema::LeaderboardInfo, 10>::vf6

/// Find a leaderboard column id in this board's id list, returning its position.
///
/// Asks the shared board-info callee (fastcall: board index in ecx, out-block pointer in edx) to
/// fill a stack out-block, then reads the entry count at `+0x0C` (`COUNT_WORD`) and the id-array
/// pointer at `+0x10` (`IDS_WORD`). If the callee reports failure, the count is zero or negative,
/// or `key` is not among the first `count` ids, returns `NOT_FOUND` (0xFFFF_FFFF); otherwise
/// returns the index of the first match.
///
/// The bound is signed (`jle`/`jl` in the original): a negative count finds nothing without
/// scanning. The scan is a plain linear search from index 0.
///
/// Original: 0x005484E0 (thiscall, one stack word; ecx is not read).
lf_checker_rt::export!(thiscall, rw_005484e0(_this: u32, key: u32) -> u32 {
    unsafe {
        const BOARD_INDEX: u32 = 0xB8;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        const COUNT_WORD: usize = 3; // +0x0C: entry count (i32)
        const IDS_WORD: usize = 4; // +0x10: id array pointer
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let _ = _this;
        let mut block = [0u32; 8];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, BOARD_INDEX, block.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return NOT_FOUND;
        }
        let count = block[COUNT_WORD];
        let ids = block[IDS_WORD];
        if (count as i32) <= 0 {
            return NOT_FOUND;
        }
        let mut i: i32 = 0;
        while i < count as i32 {
            let v = rd32(ids.wrapping_add((i as u32).wrapping_mul(4)));
            if v == key {
                return (i as u32);
            }
            i += 1;
        }
        NOT_FOUND
    }
});
