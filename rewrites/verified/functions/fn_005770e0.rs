// original: 0x005770e0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_150, player_schema::LeaderboardInfo, 10>::vf7

/// Return the leaderboard key at an unchecked index, or -1 on fetch failure.
///
/// `this` (ECX) is ignored; `index` is used as-is, with no bounds check:
/// an out-of-range index reads whatever follows the table (or faults,
/// exactly like the original). Fetches the leaderboard tables for id
/// 0x16e through the fetch callee; word 4 of the scratch structure
/// is the keys pointer. When the fetch reports failure (low byte of the
/// answer is zero) returns -1, otherwise returns `keys[index]`.
///
/// Original: 0x005770e0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_005770e0(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x16e;
        const FETCH: u32 = 0;
        const MISSING: u32 = 0xFFFF_FFFF;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = [0u32; 5];
        out[4] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32
        );
        if ok as u8 == 0 {
            return MISSING;
        }
        let keys = out[4];
        rd(keys.wrapping_add((index).wrapping_mul(4)))
    }
});
