// original: 0x005216a0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race40NoHolds, player_schema::LeaderboardInfo, 10>::vf13

/// Find a leaderboard key in the key list and map it through the value list.
///
/// `want` is the key to find. The lookup callee (id 1, leaderboard id
/// 0x7d) fills a six-word query block: key count at word 3, key-list
/// pointer at word 4, value-list pointer at word 5; only its low result byte
/// matters. Returns the value-list entry at the key's position, or
/// all-bits-set when the lookup fails, the count is zero or negative, or the
/// key is absent.
/// Edge cases: the count is signed (a huge unsigned count reads as negative
/// and finds nothing); a found index can never be -1, so the original's
/// extra check for it is dead and not reproduced.
/// Original: stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_005216a0(want: u32) -> u32 {
    unsafe {
        const LOOKUP_CALLEE: u32 = 1;
        const LEADERBOARD_ID: u32 = 0x7d;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;

        let mut block = [0u32; 6];
        let answer: u32 = lf_checker_rt::callee_fastcall!(
            LOOKUP_CALLEE,
            u32,
            LEADERBOARD_ID,
            block.as_mut_ptr() as u32
        );
        if answer & 0xFF == 0 {
            return NOT_FOUND;
        }
        let n = block[3] as i32;
        if n <= 0 {
            return NOT_FOUND;
        }
        let keys = block[4];
        let values = block[5];
        let mut i = 0i32;
        while i < n {
            let at = (i as u32).wrapping_mul(4);
            let here = (keys.wrapping_add(at) as *const u32).read_unaligned();
            if here == want {
                return (values.wrapping_add(at) as *const u32).read_unaligned();
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
