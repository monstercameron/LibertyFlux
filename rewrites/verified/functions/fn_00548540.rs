// original: 0x00548540 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_6, player_schema::LeaderboardInfo, 10>::vf7

/// Read one leaderboard row key of this board by index.
///
/// Asks the shared board-info callee (fastcall: board index in ecx, out-block pointer in edx) to
/// fill a stack out-block, then reads the row-key array pointer at `+0x10` (`KEYS_WORD`) and
/// returns `keys[index]`. Returns `NOT_FOUND` (0xFFFF_FFFF) only when the callee reports failure;
/// a stored `NOT_FOUND` value at a valid index is returned as-is and is indistinguishable from
/// the failure result.
///
/// Original: 0x00548540 (thiscall, one stack word; ecx is not read).
lf_checker_rt::export!(thiscall, rw_00548540(_this: u32, index: u32) -> u32 {
    unsafe {
        const BOARD_INDEX: u32 = 0xB8;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        const KEYS_WORD: usize = 4; // +0x10: row-key array pointer
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let _ = _this;
        let mut block = [0u32; 5];
        let ok: u32 =
            lf_checker_rt::callee_fastcall!(1, u32, BOARD_INDEX, block.as_mut_ptr() as u32);
        if ok as u8 == 0 {
            return NOT_FOUND;
        }
        let keys = block[KEYS_WORD];
        rd32(keys.wrapping_add(index.wrapping_mul(4)))
    }
});
