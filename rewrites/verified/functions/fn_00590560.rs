// original: 0x00590560 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_243, player_schema::LeaderboardInfo, 10>::vf13

/// Look up one leaderboard key in the fetched table and return its value.
///
/// Asks the row-table helper (fastcall callee 1: ECX = `LEADERBOARD_ID`,
/// EDX = out-pointer) for this leaderboard's table. The helper answers
/// nonzero on success and fills three words at the out-pointer: the row
/// `count` at `+0x0c`, the `keys` array pointer at `+0x10` and the parallel
/// `vals` array pointer at `+0x14`. The count is compared SIGNED: zero or
/// negative means not found without touching either array.
///
/// On success the keys are scanned in order for `key`; the first match
/// returns the value at the same index, and no match returns -1, as does
/// a failed fetch. `NOT_FOUND` is -1 as u32.
///
/// Original: stdcall, one stack word, callee pops 4. Incoming ECX is dead
/// (overwritten with the id before the call). The helper lives in the
/// encrypted first megabyte and never executes under the checker; its
/// call site is patched and its answers are scripted.
lf_checker_rt::export!(stdcall, rw_00590560(key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1cb;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        const OUT_COUNT: usize = 3;
        const OUT_KEYS: usize = 4;
        const OUT_VALS: usize = 5;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut query = [0u32; 8];
        query[OUT_COUNT] = 0;
        query[OUT_KEYS] = 0;
        query[OUT_VALS] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, query.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NOT_FOUND;
        }
        let count = query[OUT_COUNT] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = query[OUT_KEYS];
        let vals = query[OUT_VALS];
        let mut i = 0i32;
        while i < count {
            let at = (i as u32).wrapping_mul(4);
            if rd32(keys.wrapping_add(at)) == key {
                return rd32(vals.wrapping_add(at));
            }
            i += 1;
        }
        NOT_FOUND
    }
});
