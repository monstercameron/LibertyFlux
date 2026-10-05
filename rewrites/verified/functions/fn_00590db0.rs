// original: 0x00590DB0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_245, player_schema::LeaderboardInfo, 10>::vf12

/// Map an index through one fetched array, then find it in another.
///
/// Asks the row-table helper (fastcall callee 1: ECX = `LEADERBOARD_ID`,
/// EDX = out-pointer) for two arrays: row `count` at out `+0x04`, the
/// search `keys` at `+0x08` and the `index` array at `+0x14`. A failed
/// fetch returns -1.
///
/// On success reads `want = index[arg]` (unchecked, 32-bit wrapping
/// address) and returns -1 at once when it is -1. Then scans `keys` for
/// `want` with an UNSIGNED bound: a zero count returns -1, otherwise the
/// first match returns its index and no match returns -1. `NOT_FOUND`
/// is -1 as u32.
///
/// Original: stdcall, one stack word, callee pops 4. Incoming ECX is dead.
/// The helper's call site is patched and its answers are scripted.
lf_checker_rt::export!(stdcall, rw_00590DB0(arg: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1cd;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        const OUT_COUNT: usize = 1;
        const OUT_KEYS: usize = 2;
        const OUT_INDEX: usize = 5;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut query = [0u32; 8];
        query[OUT_COUNT] = 0;
        query[OUT_KEYS] = 0;
        query[OUT_INDEX] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, query.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NOT_FOUND;
        }
        let index = query[OUT_INDEX];
        let want = rd32(index.wrapping_add(arg.wrapping_mul(4)));
        if want == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = query[OUT_COUNT];
        if count == 0 {
            return NOT_FOUND;
        }
        let keys = query[OUT_KEYS];
        let mut i = 0u32;
        loop {
            if rd32(keys.wrapping_add(i.wrapping_mul(4))) == want {
                return i;
            }
            i = i.wrapping_add(1);
            if i >= count {
                return NOT_FOUND;
            }
        }
    }
});
