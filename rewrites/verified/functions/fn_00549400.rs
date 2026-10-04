// original: 0x00549400 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_10, player_schema::LeaderboardInfo, 10>::vf12

/// Map a leaderboard row index to its position in this board's id list.
///
/// Asks the shared board-info callee (fastcall: board index in ecx, out-block pointer in edx) to
/// fill a stack out-block, then reads the entry count at `+0x04` (`COUNT_WORD`), the id-array
/// pointer at `+0x08` (`IDS_WORD`) and the row-key array pointer at `+0x14` (`KEYS_WORD`). The key
/// `keys[index]` is looked up in the first `count` ids and the position of the first match is
/// returned. Returns `NOT_FOUND` (0xFFFF_FFFF) when the callee reports failure, the row key is
/// itself `NOT_FOUND`, the count is zero, or the key is absent.
///
/// Unlike vf6/vf13 the bound here is unsigned (`jb` in the original), and the searched key comes
/// from the row array rather than from the caller.
///
/// Original: 0x00549400 (thiscall, one stack word; ecx is not read).
lf_checker_rt::export!(thiscall, rw_00549400(_this: u32, index: u32) -> u32 {
    unsafe {
        const BOARD_INDEX: u32 = 0xBC;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        const COUNT_WORD: usize = 1; // +0x04: entry count (u32)
        const IDS_WORD: usize = 2; // +0x08: id array pointer
        const KEYS_WORD: usize = 5; // +0x14: row-key array pointer
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
        let keys = block[KEYS_WORD];
        let want = rd32(keys.wrapping_add(index.wrapping_mul(4)));
        if want == NOT_FOUND {
            return NOT_FOUND;
        }
        if count == 0 {
            return NOT_FOUND;
        }
        let mut i = 0u32;
        while i < count {
            let v = rd32(ids.wrapping_add(i.wrapping_mul(4)));
            if v == want {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
