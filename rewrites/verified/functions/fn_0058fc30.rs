// original: 0x0058FC30 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_241, player_schema::LeaderboardInfo, 10>::vf12
/// Leaderboard cross lookup: find the key at `index` of the first table
/// inside the second table. Calls the leaderboard helper (id 0x1c9)
/// with a six-word scratch record; on success it leaves the second-table
/// count at word 1 (`+0x04`), the second-table pointer at word 2 (`+0x08`)
/// and the first-table pointer at word 5 (`+0x14`). A first-table key of
/// 0xFFFFFFFF, an empty second table, or no match yields 0xFFFFFFFF,
/// otherwise the zero-based position in the second table. The count loop
/// is unsigned.
/// Original: 0x0058FC30 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_0058fc30(index: u32) -> u32 {
    unsafe {
        const RACE_ID: u32 = 0x1c9;
        const COUNT_WORD: usize = 1;
        const OTHER_WORD: usize = 2;
        const KEYS_WORD: usize = 5;
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
        let elem = rd32(info[KEYS_WORD].wrapping_add(index.wrapping_mul(4)));
        if elem == MISS {
            return MISS;
        }
        let count = info[COUNT_WORD];
        if count == 0 {
            return MISS;
        }
        let other = info[OTHER_WORD];
        let mut i = 0u32;
        loop {
            if rd32(other.wrapping_add(i.wrapping_mul(4))) == elem {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return MISS;
            }
        }
    }
});
