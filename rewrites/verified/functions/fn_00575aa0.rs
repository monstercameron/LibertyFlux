// original: 0x00575aa0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_145, player_schema::LeaderboardInfo, 10>::vf6

/// Find a leaderboard key and return its index, or -1 when absent.
///
/// `this` (ECX) is ignored; `key` is the value to find. Fetches the
/// leaderboard tables for id 0x169 through the fetch callee into a
/// scratch structure, of which words 3..5 are the (count, keys) pair used
/// here. When the fetch reports failure (low byte of the answer is zero)
/// or the count is not positive, returns -1. Otherwise linearly scans
/// `keys[0..count]` for `key` (signed bound) and returns the first
/// matching index, or -1 when no element matches.
///
/// Original: 0x00575aa0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00575aa0(_this: u32, key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x169;
        const FETCH: u32 = 0;
        const MISSING: u32 = 0xFFFF_FFFF;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = [0u32; 6];
        out[3] = 0;
        out[4] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32
        );
        if ok as u8 == 0 {
            return MISSING;
        }
        let count = out[3] as i32;
        if count <= 0 {
            return MISSING;
        }
        let keys = out[4];
        let mut i = 0u32;
        while i < count as u32 {
            if rd(keys.wrapping_add(i.wrapping_mul(4))) == key {
                return i;
            }
            i = i.wrapping_add(1);
        }
        MISSING
    }
});
