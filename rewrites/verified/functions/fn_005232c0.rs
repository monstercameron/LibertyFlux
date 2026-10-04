// original: 0x005232c0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race46NoHolds, player_schema::LeaderboardInfo, 10>::vf6

/// Find a leaderboard key in the key list.
///
/// `want` is the key to find. The lookup callee (id 1, leaderboard id
/// 0x1f) fills a five-word query block: key count at word 3, key-list
/// pointer at word 4; only its low result byte matters. Returns the key's
/// index, or all-bits-set when the lookup fails, the count is zero or
/// negative, or the key is absent.
/// Edge cases: the count is signed.
/// Original: stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_005232c0(want: u32) -> u32 {
    unsafe {
        const LOOKUP_CALLEE: u32 = 1;
        const LEADERBOARD_ID: u32 = 0x1f;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;

        let mut block = [0u32; 5];
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
        let mut i = 0i32;
        while i < n {
            let at = (i as u32).wrapping_mul(4);
            let here = (keys.wrapping_add(at) as *const u32).read_unaligned();
            if here == want {
                return i as u32;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
