// original: 0x00589780 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_218, player_schema::LeaderboardInfo, 10>::vf12

/// Reverse column-index lookup for one ranked episodic-race leaderboard.
///
/// Asks the leaderboard helper (callee 1) for the column lists of leaderboard
/// id 0x1b2: an index array at buffer byte 20 and a search array at byte
/// 8 with its (unsigned) count at byte 4. Takes element `index` of
/// the index array and returns its position in the search array, or -1 when
/// the helper fails, the element is -1 (empty slot), the count is zero, or
/// the value is absent. The index into the first array is not bounds-checked.
///
/// The helper takes the leaderboard id in ECX and an out-buffer in EDX and
/// answers in AL. This method ignores its `this` pointer and takes one stack
/// argument (`index`). Original is stdcall (the callee pops 4 bytes); the count comparison
/// is unsigned.

lf_checker_rt::export!(stdcall, rw_00589780(index: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const LEADERBOARD_ID: u32 = 0x1b2;
        const HELPER: u32 = 1;
        const COUNT_WORD: usize = 1;
        const SEARCH_WORD: usize = 2;
        const INDEX_WORD: usize = 5;
        const EMPTY: u32 = 0xffff_ffff;
        const NOT_FOUND: u32 = 0xffff_ffff;

        let mut info = [0u32; 6];
        info[COUNT_WORD] = 0;
        info[SEARCH_WORD] = 0;
        info[INDEX_WORD] = 0;
        let ok: u8 = lf_checker_rt::callee_fastcall!(HELPER, u8, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let cell = info[INDEX_WORD].wrapping_add(index.wrapping_mul(4));
        let want = rd32(cell);
        if want == EMPTY {
            return NOT_FOUND;
        }
        let count = info[COUNT_WORD];
        if count == 0 {
            return NOT_FOUND;
        }
        let table = info[SEARCH_WORD];
        let mut at = 0u32;
        while at < count {
            if rd32(table.wrapping_add(at.wrapping_mul(4))) == want {
                return at;
            }
            at += 1;
        }
        NOT_FOUND
    }
});
