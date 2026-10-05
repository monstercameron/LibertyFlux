// original: 0x005897f0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_218, player_schema::LeaderboardInfo, 10>::vf13

/// Column-value gather for one ranked episodic-race leaderboard.
///
/// Asks the leaderboard helper (callee 1) for the column lists of leaderboard
/// id 0x1b2: a key array at buffer byte 16 with its (signed) count at
/// byte 12, and a parallel value array at byte 20. Scans the key
/// array for `key` and returns the value at the same position, or -1 when
/// the helper fails, the count is not positive, or the key is absent.
///
/// The helper takes the leaderboard id in ECX and an out-buffer in EDX and
/// answers in AL. This method ignores its `this` pointer and takes one stack
/// argument (`key`). Original is stdcall (the callee pops 4 bytes). (The original re-tests
/// the found index against -1 after the scan; that branch is unreachable --
/// the scan only exits there with an index in range -- so it is not
/// reproduced.)

lf_checker_rt::export!(stdcall, rw_005897f0(key: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const LEADERBOARD_ID: u32 = 0x1b2;
        const HELPER: u32 = 1;
        const COUNT_WORD: usize = 3;
        const KEYS_WORD: usize = 4;
        const VALUES_WORD: usize = 5;
        const NOT_FOUND: u32 = 0xffff_ffff;

        let mut info = [0u32; 6];
        info[COUNT_WORD] = 0;
        info[KEYS_WORD] = 0;
        info[VALUES_WORD] = 0;
        let ok: u8 = lf_checker_rt::callee_fastcall!(HELPER, u8, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if ok == 0 {
            return NOT_FOUND;
        }
        let count = info[COUNT_WORD] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = info[KEYS_WORD];
        let mut index = 0i32;
        while index < count {
            if rd32(keys.wrapping_add((index as u32).wrapping_mul(4))) == key {
                let values = info[VALUES_WORD];
                return rd32(values.wrapping_add((index as u32).wrapping_mul(4)));
            }
            index += 1;
        }
        NOT_FOUND
    }
});
