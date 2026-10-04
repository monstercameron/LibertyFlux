// original: 0x0058FCA0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_241, player_schema::LeaderboardInfo, 10>::vf13
/// Leaderboard mapped lookup: find `wanted` in the key table and return
/// the value at the same position of the parallel value table. Calls the
/// leaderboard helper (id 0x1c9) with a six-word scratch record; on
/// success it leaves the entry count at word 3 (`+0x0c`), the key-table
/// pointer at word 4 (`+0x10`) and the value-table pointer at word 5
/// (`+0x14`). A failed helper call, a non-positive count, or no matching
/// key yields 0xFFFFFFFF. The count is treated as signed.
/// Original: 0x0058FCA0 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0058fca0(wanted: u32) -> u32 {
    unsafe {
        const RACE_ID: u32 = 0x1c9;
        const COUNT_WORD: usize = 3;
        const KEYS_WORD: usize = 4;
        const VALS_WORD: usize = 5;
        const MISS: u32 = 0xFFFF_FFFF;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut info = [0u32; 6];
        let ok = lf_checker_rt::callee_fastcall!(1, u32, RACE_ID, info.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return MISS;
        }
        let count = info[COUNT_WORD] as i32;
        if count <= 0 {
            return MISS;
        }
        let keys = info[KEYS_WORD];
        let mut found = -1i32;
        let mut i = 0i32;
        while i < count {
            if rd32(keys.wrapping_add((i as u32).wrapping_mul(4))) == wanted {
                found = i;
                break;
            }
            i += 1;
        }
        if found < 0 {
            return MISS;
        }
        rd32(info[VALS_WORD].wrapping_add((found as u32).wrapping_mul(4)))
    }
});
