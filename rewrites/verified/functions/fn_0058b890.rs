// original: 0x0058b890 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_225, player_schema::LeaderboardInfo, 10>::vf6

/// Column-index lookup for one ranked episodic-race leaderboard.
///
/// Asks the leaderboard helper (callee 1) for the column list of leaderboard
/// id 0x1b9, then linearly scans the returned array for `key` and returns
/// its index, or -1 when the helper reports failure, the count is not
/// positive, or the key is absent.
///
/// The helper takes the leaderboard id in ECX and an out-buffer in EDX and
/// answers in AL; on success it fills the count at buffer byte 12 and the
/// array pointer at byte 16. This method ignores its `this` pointer and
/// takes one stack argument (`key`). Original is stdcall (the callee pops 4 bytes).

lf_checker_rt::export!(stdcall, rw_0058b890(key: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const LEADERBOARD_ID: u32 = 0x1b9;
        const HELPER: u32 = 1;
        const COUNT_WORD: usize = 3;
        const ARRAY_WORD: usize = 4;
        const NOT_FOUND: u32 = 0xffff_ffff;

        let mut info = [0u32; 6];
        info[COUNT_WORD] = 0;
        info[ARRAY_WORD] = 0;
        let ok: u8 = lf_checker_rt::callee_fastcall!(HELPER, u8, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let count = info[COUNT_WORD] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let array = info[ARRAY_WORD];
        let mut index = 0i32;
        while index < count {
            let cell = array.wrapping_add((index as u32).wrapping_mul(4));
            if rd32(cell) == key {
                return index as u32;
            }
            index += 1;
        }
        NOT_FOUND
    }
});
