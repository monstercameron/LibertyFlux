// original: 0x00521a90 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race41NoHolds, player_schema::LeaderboardInfo, 10>::vf12

/// Look up one leaderboard key by position and find it in the key list.
///
/// `key_index` selects an entry of the value list. The lookup callee
/// (id 1, called with leaderboard id 0x7e) fills a six-word query block on
/// the stack: the key count at word 1, the key-list pointer at word 2 and
/// the value-list pointer at word 5; only its low result byte matters.
/// Returns the key's index in the key list, or all-bits-set when the lookup
/// fails, the selected entry holds the missing-key marker, the list is empty
/// or the key is absent.
/// Edge cases: a zero low byte with nonzero upper bytes fails like zero;
/// the count comparison is unsigned.
/// Original: stdcall, one stack word.
lf_checker_rt::export!(stdcall, rw_00521a90(key_index: u32) -> u32 {
    unsafe {
        const LOOKUP_CALLEE: u32 = 1;
        const LEADERBOARD_ID: u32 = 0x7e;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        const NO_KEY: u32 = 0xFFFF_FFFF;

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
        let count = block[1];
        let keys = block[2];
        let key = (block[5].wrapping_add(key_index.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        if key == NO_KEY {
            return NOT_FOUND;
        }
        let mut i = 0u32;
        while i < count {
            let here =
                (keys.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if here == key {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
