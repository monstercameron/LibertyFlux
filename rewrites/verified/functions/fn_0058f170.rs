// original: 0x0058F170 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_238, player_schema::LeaderboardInfo, 10>::vf6
/// Leaderboard key lookup: find `wanted` in the race's key table.
/// Calls the leaderboard helper (id 0x1c6) with a five-word scratch
/// record; on success the helper leaves the entry count at record word 3
/// (`+0x0c`) and the key-table pointer at word 4 (`+0x10`). Returns the
/// zero-based index of the first matching key, or 0xFFFFFFFF when the
/// helper fails, the count is not positive, or no key matches. The count
/// is treated as signed: a negative count misses without reading the table.
/// Original: 0x0058F170 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0058f170(wanted: u32) -> u32 {
    unsafe {
        const RACE_ID: u32 = 0x1c6;
        const COUNT_WORD: usize = 3;
        const KEYS_WORD: usize = 4;
        const MISS: u32 = 0xFFFF_FFFF;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut info = [0u32; 5];
        let ok = lf_checker_rt::callee_fastcall!(1, u32, RACE_ID, info.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return MISS;
        }
        let count = info[COUNT_WORD] as i32;
        if count <= 0 {
            return MISS;
        }
        let keys = info[KEYS_WORD];
        let mut i = 0i32;
        while i < count {
            if rd32(keys.wrapping_add((i as u32).wrapping_mul(4))) == wanted {
                return i as u32;
            }
            i += 1;
        }
        MISS
    }
});
