// original: 0x00549010 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_9, player_schema::LeaderboardInfo, 10>::vf13

/// Look up a leaderboard column id in this board's id list, returning the paired value.
///
/// Asks the shared board-info callee (fastcall: board index in ecx, out-block pointer in edx) to
/// fill a stack out-block, then reads the entry count at `+0x0C` (`COUNT_WORD`), the id-array
/// pointer at `+0x10` (`IDS_WORD`) and the value-array pointer at `+0x14` (`VALS_WORD`). If the
/// callee reports failure, the count is zero or negative, or `key` is not among the first `count`
/// ids, returns `NOT_FOUND` (0xFFFF_FFFF); otherwise returns the value paired with the first
/// matching id.
///
/// The bound is signed like vf6's. The original re-checks the found index against -1 after the
/// loop; that check is dead (a loop counter starting at 0 cannot be -1) and is not reproduced.
///
/// Original: 0x00549010 (thiscall, one stack word; ecx is not read).
lf_checker_rt::export!(thiscall, rw_00549010(_this: u32, key: u32) -> u32 {
    unsafe {
        const BOARD_INDEX: u32 = 0xBB;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        const COUNT_WORD: usize = 3; // +0x0C: entry count (i32)
        const IDS_WORD: usize = 4; // +0x10: id array pointer
        const VALS_WORD: usize = 5; // +0x14: value array pointer
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
        let vals = block[VALS_WORD];
        if (count as i32) <= 0 {
            return NOT_FOUND;
        }
        let mut i: i32 = 0;
        while i < count as i32 {
            let v = rd32(ids.wrapping_add((i as u32).wrapping_mul(4)));
            if v == key {
                return rd32(vals.wrapping_add((i as u32).wrapping_mul(4)));
            }
            i += 1;
        }
        NOT_FOUND
    }
});
