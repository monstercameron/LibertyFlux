// original: 0x00591010 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_245, player_schema::LeaderboardInfo, 10>::vf6

/// Look up one leaderboard key in the fetched table and return its index.
///
/// Same fetch as the sibling value lookup (fastcall callee 1: ECX =
/// `LEADERBOARD_ID`, EDX = out-pointer; nonzero answer means success;
/// row `count` at out `+0x0c`, `keys` array at `+0x10`). The count is
/// compared SIGNED: zero or negative returns -1 without reading keys.
///
/// On success the keys are scanned in order for `key`; the first match
/// returns its zero-based index, and no match returns -1, as does a
/// failed fetch. `NOT_FOUND` is -1 as u32.
///
/// Original: stdcall, one stack word, callee pops 4. Incoming ECX is dead.
/// The helper's call site is patched and its answers are scripted.
lf_checker_rt::export!(stdcall, rw_00591010(key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1cd;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        const OUT_COUNT: usize = 3;
        const OUT_KEYS: usize = 4;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut query = [0u32; 8];
        query[OUT_COUNT] = 0;
        query[OUT_KEYS] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, query.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NOT_FOUND;
        }
        let count = query[OUT_COUNT] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = query[OUT_KEYS];
        let mut i = 0i32;
        while i < count {
            if rd32(keys.wrapping_add((i as u32).wrapping_mul(4))) == key {
                return i as u32;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
