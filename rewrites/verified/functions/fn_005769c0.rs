// original: 0x005769c0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_149, player_schema::LeaderboardInfo, 10>::vf12

/// Map a leaderboard value slot back to its key index, or -1 when absent.
///
/// `this` (ECX) is ignored; `index` addresses the values table with no
/// bounds check. Fetches the leaderboard tables for id 0x16d
/// through the fetch callee into a six-word scratch structure: words 1..3
/// are the first (count, keys) pair, word 5 is the values pointer. When
/// the fetch fails (low byte of the answer is zero) returns -1. Otherwise
/// reads `want = values[index]`; when that is -1 returns -1 without
/// searching. When the count is zero returns -1; otherwise linearly scans
/// `keys[0..count]` for `want` with an unsigned bound and returns the
/// first matching index, or -1 when no element matches.
///
/// Original: 0x005769c0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_005769c0(_this: u32, index: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x16d;
        const FETCH: u32 = 0;
        const MISSING: u32 = 0xFFFF_FFFF;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut out = [0u32; 6];
        out[1] = 0;
        out[2] = 0;
        out[5] = 0;
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            FETCH, u32, LEADERBOARD_ID, out.as_mut_ptr() as u32
        );
        if ok as u8 == 0 {
            return MISSING;
        }
        let vals = out[5];
        let want = rd(vals.wrapping_add(index.wrapping_mul(4)));
        if want == MISSING {
            return MISSING;
        }
        let count = out[1];
        if count == 0 {
            return MISSING;
        }
        let keys = out[2];
        let mut i = 0u32;
        while i < count {
            if rd(keys.wrapping_add(i.wrapping_mul(4))) == want {
                return i;
            }
            i = i.wrapping_add(1);
        }
        MISSING
    }
});
