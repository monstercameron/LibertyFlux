// original: 0x00549B60 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_11, player_schema::LeaderboardInfo, 10>::vf8

/// Return the display width of one leaderboard column of this board.
///
/// Asks the shared board-info callee (fastcall: board index in ecx, out-block pointer in edx) to
/// fill a stack out-block, then reads the column-descriptor array pointer at `+0x14`
/// (`COLS_WORD`). The descriptor `cols[index]` is classified by a second callee (thiscall, no
/// stack arguments) into a column kind; the kind selects a width through the original's jump
/// table: kind 1 maps to 4, kinds 2-3 to 8, kind 5 to 4, and kind 4, kind -1, a callee failure,
/// or anything above 5 to 0.
///
/// Original: 0x00549B60 (thiscall, one stack word; ecx is not read).
lf_checker_rt::export!(thiscall, rw_00549b60(_this: u32, index: u32) -> u32 {
    unsafe {
        const BOARD_INDEX: u32 = 0xBD;
        const COLS_WORD: usize = 5; // +0x14: column-descriptor array pointer
        const WIDTH_NARROW: u32 = 4;
        const WIDTH_WIDE: u32 = 8;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let _ = _this;
        let mut block = [0u32; 6];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, BOARD_INDEX, block.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return 0;
        }
        let cols = block[COLS_WORD];
        let column = rd32(cols.wrapping_add(index.wrapping_mul(4)));
        let kind: u32 = lf_checker_rt::callee_thiscall!(2, u32, column);
        if kind == 0xFFFF_FFFF {
            return 0;
        }
        match kind.wrapping_sub(1) {
            0 => WIDTH_NARROW,
            1 | 2 => WIDTH_WIDE,
            4 => WIDTH_NARROW,
            _ => 0,
        }
    }
});
