// original: 0x00590350 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_242, player_schema::LeaderboardInfo, 10>::vf7

/// Return one entry of the fetched leaderboard array by index.
///
/// Asks the row-table helper (fastcall callee 1: ECX = `LEADERBOARD_ID`,
/// EDX = out-pointer) for this leaderboard's array; the helper answers
/// nonzero on success and fills the array pointer at out `+0x10`. On a
/// failed fetch returns -1.
///
/// On success returns `array[idx]` with NO bounds check: the index is
/// scaled by 4 and added to the base with 32-bit wrapping, exactly as
/// the original's indexed load. `NOT_FOUND` is -1 as u32.
///
/// Original: stdcall, one stack word, callee pops 4. Incoming ECX is dead.
/// The helper's call site is patched and its answers are scripted.
lf_checker_rt::export!(stdcall, rw_00590350(idx: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x1ca;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        const OUT_ARR: usize = 4;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut query = [0u32; 8];
        query[OUT_ARR] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, LEADERBOARD_ID, query.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NOT_FOUND;
        }
        let arr = query[OUT_ARR];
        rd32(arr.wrapping_add(idx.wrapping_mul(4)))
    }
});
